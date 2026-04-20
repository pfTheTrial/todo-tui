use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PomodoroProfile {
    pub name: String,
    pub work_duration: u32,  // Minutes
    pub short_break: u32,    // Minutes
    pub long_break: u32,     // Minutes
}

impl Default for PomodoroProfile {
    fn default() -> Self {
        Self {
            name: "Default".into(),
            work_duration: 25,
            short_break: 5,
            long_break: 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pomodoro {
    #[serde(default = "default_profiles")]
    pub profiles: Vec<PomodoroProfile>,
    #[serde(default)]
    pub active_profile_index: usize,
    
    pub sessions_until_long: u32,
    pub current_session_count: u32,
    #[serde(default)]
    pub total_sessions_completed: u32,
    
    // Runtime state
    pub remaining_seconds: u32,
    pub is_running: bool,
    pub phase: PomodoroPhase,
}

fn default_profiles() -> Vec<PomodoroProfile> {
    vec![
        PomodoroProfile { name: "Tradicional".into(), work_duration: 25, short_break: 5, long_break: 15 },
        PomodoroProfile { name: "Focus 50".into(), work_duration: 50, short_break: 10, long_break: 30 },
        PomodoroProfile { name: "Rápido".into(), work_duration: 15, short_break: 2, long_break: 5 },
    ]
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak,
}

impl Default for Pomodoro {
    fn default() -> Self {
        Self {
            profiles: default_profiles(),
            active_profile_index: 0,
            sessions_until_long: 4,
            current_session_count: 0,
            total_sessions_completed: 0,
            remaining_seconds: 25 * 60,
            is_running: false,
            phase: PomodoroPhase::Work,
        }
    }
}

impl Pomodoro {
    pub fn tick(&mut self) -> bool {
        if !self.is_running || self.remaining_seconds == 0 {
            return false;
        }

        self.remaining_seconds -= 1;
        
        if self.remaining_seconds == 0 {
            self.is_running = false;
            return true; // Finished
        }
        false
    }

    pub fn reset(&mut self) {
        let profile = self.profiles.get(self.active_profile_index).cloned().unwrap_or_default();
        self.remaining_seconds = match self.phase {
            PomodoroPhase::Work => profile.work_duration * 60,
            PomodoroPhase::ShortBreak => profile.short_break * 60,
            PomodoroPhase::LongBreak => profile.long_break * 60,
        };
        self.is_running = false;
    }

    pub fn next_phase(&mut self) {
        match self.phase {
            PomodoroPhase::Work => {
                self.current_session_count += 1;
                if self.current_session_count % self.sessions_until_long == 0 {
                    self.phase = PomodoroPhase::LongBreak;
                } else {
                    self.phase = PomodoroPhase::ShortBreak;
                }
            }
            PomodoroPhase::ShortBreak | PomodoroPhase::LongBreak => {
                self.phase = PomodoroPhase::Work;
            }
        }
        self.reset();
    }

    pub fn force_break(&mut self) {
        if self.phase == PomodoroPhase::Work {
            if self.current_session_count > 0 && self.current_session_count % self.sessions_until_long == 0 {
                self.phase = PomodoroPhase::LongBreak;
            } else {
                self.phase = PomodoroPhase::ShortBreak;
            }
            self.reset();
        }
    }

    pub fn progress(&self) -> f64 {
        let profile = self.profiles.get(self.active_profile_index).cloned().unwrap_or_default();
        let total = match self.phase {
            PomodoroPhase::Work => profile.work_duration * 60,
            PomodoroPhase::ShortBreak => profile.short_break * 60,
            PomodoroPhase::LongBreak => profile.long_break * 60,
        } as f64;
        
        if total == 0.0 { return 1.0; }
        (total - self.remaining_seconds as f64) / total
    }
}
