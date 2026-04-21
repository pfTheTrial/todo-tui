use std::io::Read;

pub fn get_platform_asset_name() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => "todo-tui-windows-amd64.exe",
        ("linux", "x86_64")   => "todo-tui-linux-amd64",
        ("macos", "aarch64")  => "todo-tui-macos-arm64",
        ("macos", "x86_64")   => "todo-tui-macos-amd64",
        _ => "",
    }
}

pub fn perform_update(version: &str) -> Result<String, String> {
    let asset = get_platform_asset_name();
    if asset.is_empty() {
        return Err("unsupported_platform".into());
    }
    
    let url = format!(
        "https://github.com/pfTheTrial/todo-tui/releases/download/v{}/{}",
        version, asset
    );
    
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("exe lookup: {}", e))?;
    
    let tmp_path = current_exe.with_extension("new");
    
    // Download
    let agent = ureq::Agent::new_with_defaults();
    let mut response = agent.get(&url)
        .header("User-Agent", "tdt-auto-updater")
        .call()
        .map_err(|e| format!("download: {}", e))?;
    
    let mut bytes = Vec::new();
    response.body_mut().as_reader().read_to_end(&mut bytes)
        .map_err(|e| format!("read: {}", e))?;
    
    std::fs::write(&tmp_path, &bytes)
        .map_err(|e| format!("write: {}", e))?;
    
    // Platform-specific replacement
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("chmod: {}", e))?;
        std::fs::rename(&tmp_path, &current_exe)
            .map_err(|e| format!("rename: {}", e))?;
    }
    
    #[cfg(windows)]
    {
        let old_path = current_exe.with_extension("old");
        let _ = std::fs::remove_file(&old_path); // Clean up previous .old
        std::fs::rename(&current_exe, &old_path)
            .map_err(|e| format!("rename current: {}", e))?;
        std::fs::rename(&tmp_path, &current_exe)
            .map_err(|e| format!("rename new: {}", e))?;
    }
    
    Ok(version.to_string())
}
