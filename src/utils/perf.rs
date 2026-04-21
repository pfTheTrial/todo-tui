/// Lightweight self-resource sampler — no external crates needed.
/// On Windows uses GetProcessMemoryInfo via kernel32.
/// On Linux reads /proc/self/status.
/// Returns (ram_mb, cpu_pct_approx).
pub fn sample_self() -> (f32, f32) {
    let ram = get_ram_mb();
    let cpu = get_cpu_pct();
    (ram, cpu)
}

#[cfg(windows)]
fn get_ram_mb() -> f32 {
    // Use kernel32 PROCESS_MEMORY_COUNTERS via raw FFI (no extra crate)
    // Working set = physical pages currently in use
    use std::mem;

    #[repr(C)]
    #[allow(non_snake_case, non_camel_case_types)]
    struct PROCESS_MEMORY_COUNTERS {
        cb: u32,
        PageFaultCount: u32,
        PeakWorkingSetSize: usize,
        WorkingSetSize: usize,
        QuotaPeakPagedPoolUsage: usize,
        QuotaPagedPoolUsage: usize,
        QuotaPeakNonPagedPoolUsage: usize,
        QuotaNonPagedPoolUsage: usize,
        PagefileUsage: usize,
        PeakPagefileUsage: usize,
    }

    extern "system" {
        fn GetCurrentProcess() -> *mut u8;
        fn K32GetProcessMemoryInfo(
            process: *mut u8,
            counters: *mut PROCESS_MEMORY_COUNTERS,
            size: u32,
        ) -> i32;
    }

    let mut pmc: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
    pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
    let ok = unsafe { K32GetProcessMemoryInfo(GetCurrentProcess(), &mut pmc, pmc.cb) };
    if ok != 0 {
        pmc.WorkingSetSize as f32 / 1_048_576.0
    } else {
        0.0
    }
}

#[cfg(not(windows))]
fn get_ram_mb() -> f32 {
    // /proc/self/status — VmRSS line gives RSS in kB
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let kb: f32 = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
                return kb / 1024.0;
            }
        }
    }
    0.0
}

/// CPU % approximation: measures process CPU time over a short interval.
/// Called once per second in the tick loop; compares delta with wall clock delta.
/// Stored as app state between calls.
fn get_cpu_pct() -> f32 {
    #[cfg(windows)]
    {
        use std::mem;

        #[repr(C)]
        #[allow(non_snake_case)]
        struct FILETIME {
            dwLowDateTime: u32,
            dwHighDateTime: u32,
        }

        extern "system" {
            fn GetCurrentProcess() -> *mut u8;
            fn GetProcessTimes(
                hProcess: *mut u8,
                lpCreationTime: *mut FILETIME,
                lpExitTime: *mut FILETIME,
                lpKernelTime: *mut FILETIME,
                lpUserTime: *mut FILETIME,
            ) -> i32;
            fn GetSystemInfo(lpSystemInfo: *mut SystemInfo);
        }

        #[repr(C)]
        #[allow(non_snake_case, non_camel_case_types)]
        struct SystemInfo {
            wProcessorArchitecture: u16,
            wReserved: u16,
            dwPageSize: u32,
            lpMinimumApplicationAddress: *mut u8,
            lpMaximumApplicationAddress: *mut u8,
            dwActiveProcessorMask: usize,
            dwNumberOfProcessors: u32,
            dwProcessorType: u32,
            dwAllocationGranularity: u32,
            wProcessorLevel: u16,
            wProcessorRevision: u16,
        }

        unsafe {
            let proc = GetCurrentProcess();
            let mut creation = mem::zeroed::<FILETIME>();
            let mut exit = mem::zeroed::<FILETIME>();
            let mut kernel = mem::zeroed::<FILETIME>();
            let mut user = mem::zeroed::<FILETIME>();

            static mut LAST_CPU_100NS: u64 = 0;
            static mut LAST_WALL_NS: u64 = 0;

            GetProcessTimes(proc, &mut creation, &mut exit, &mut kernel, &mut user);
            let cpu_now = ((kernel.dwHighDateTime as u64) << 32 | kernel.dwLowDateTime as u64)
                .wrapping_add((user.dwHighDateTime as u64) << 32 | user.dwLowDateTime as u64);

            let wall_now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);

            let cpu_delta = cpu_now.wrapping_sub(LAST_CPU_100NS);
            let wall_delta = wall_now.wrapping_sub(LAST_WALL_NS);

            LAST_CPU_100NS = cpu_now;
            LAST_WALL_NS = wall_now;

            let mut sysinfo: SystemInfo = mem::zeroed();
            GetSystemInfo(&mut sysinfo);
            let ncpu = sysinfo.dwNumberOfProcessors.max(1) as f32;

            if wall_delta == 0 {
                return 0.0;
            }
            // cpu_delta is in 100ns units, wall_delta in ns
            let pct = (cpu_delta as f32 * 100.0) / (wall_delta as f32 / 100.0) / ncpu;
            pct.min(100.0).max(0.0)
        }
    }

    #[cfg(not(windows))]
    {
        // /proc/self/stat — fields 14 (utime) + 15 (stime) in clock ticks
        use std::time::Instant;
        static mut LAST_TICKS: u64 = 0;
        static mut LAST_INSTANT: Option<Instant> = None;

        let clk_tck = 100u64; // Hz (sysconf _SC_CLK_TCK typically 100)
        if let Ok(stat) = std::fs::read_to_string("/proc/self/stat") {
            let fields: Vec<&str> = stat.split_whitespace().collect();
            if fields.len() > 15 {
                let utime: u64 = fields[13].parse().unwrap_or(0);
                let stime: u64 = fields[14].parse().unwrap_or(0);
                let ticks = utime + stime;
                let now = Instant::now();
                unsafe {
                    let delta_ticks = ticks.wrapping_sub(LAST_TICKS);
                    let delta_secs = LAST_INSTANT.map_or(1.0, |p| now.duration_since(p).as_secs_f32());
                    LAST_TICKS = ticks;
                    LAST_INSTANT = Some(now);
                    let pct = (delta_ticks as f32 / clk_tck as f32 / delta_secs) * 100.0;
                    return pct.min(100.0).max(0.0);
                }
            }
        }
        0.0
    }
}
