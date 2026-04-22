use crate::i18n::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme_name: String,
    pub notifications_enabled: bool,
    pub startup_with_windows: bool,
    pub language: Language,
    pub sync_interval_mins: u32,
    pub notion_api_key: Option<String>,
    pub notion_database_id: Option<String>,
    pub is_first_run: bool,
    #[serde(default = "default_true")]
    pub sync_enabled: bool,
    #[serde(default = "default_true")]
    pub task_reminders_enabled: bool,
    #[serde(default)]
    pub last_daily_digest: Option<String>,
}

fn default_true() -> bool {
    true
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme_name: "Dracula".into(),
            notifications_enabled: true,
            startup_with_windows: false,
            language: crate::i18n::detect_system_language(),
            sync_interval_mins: 10,
            notion_api_key: None,
            notion_database_id: None,
            is_first_run: true,
            sync_enabled: true,
            task_reminders_enabled: true,
            last_daily_digest: None,
        }
    }
}
