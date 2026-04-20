use crate::model::{Task, Pomodoro, settings::AppSettings};
use crate::i18n::{I18n, Language};
use serde::{Serialize, Deserialize};
use crate::storage::JsonStore;
use color_eyre::Result;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistentState {
    pub pomodoro: Pomodoro,
    pub settings: AppSettings,
}

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    EditingTitle,
    EditingDescription,
    EditingDate,
    EditingReview,
    CreatingTitle,
    CreatingDescription,
    CreatingDate,
    CreatingReview,
    EditingPomodoro,
    ConfirmingDelete,
    MenuSettings,
    MenuSync,
    EditingNotionKey,
    EditingNotionDatabase,
    EditingSyncInterval,
    Filtering,
    ImportingExcel,
    Onboarding,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SortMode {
    Priority,
    Date,
    Title,
}

#[derive(Debug, PartialEq)]
pub enum ActivePanel {
    TaskList,
    TaskDetail,
    Pomodoro,
}

pub struct App {
    pub tasks: Vec<Task>,
    pub selected_task_index: usize,
    pub input_mode: InputMode,
    pub active_panel: ActivePanel,
    pub store: JsonStore,
    pub input_buffer: String,
    pub show_help: bool,
    pub wizard_title: String,
    pub wizard_desc: String,
    pub wizard_date: String,
    pub pomodoro: Pomodoro,
    pub task_list_state: ratatui::widgets::ListState,
    pub pomodoro_store: JsonStore,
    pub theme: crate::ui::theme::Theme,
    pub settings: AppSettings,
    pub i18n: I18n,
    pub menu_cursor: usize,
    pub detail_scroll_offset: u16,
    pub status_message: Option<String>,
    pub update_info: Option<crate::utils::update::UpdateInfo>,
    #[allow(dead_code)]
    pub last_sync: std::time::Instant,
    pub filter_text: String,
    pub sort_mode: SortMode,
    pub frame_count: u64,
    pub onboarding_index: usize,
}

impl App {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        let tasks_path = data_dir.join("tasks.json");
        let pomo_path = data_dir.join("pomodoro.json");
        
        let store = JsonStore::new(tasks_path);
        let tasks: Vec<Task> = if store.exists() {
            store.load()?
        } else {
            Vec::new()
        };

        let pomodoro_store = JsonStore::new(pomo_path);
        let (pomodoro, settings) = if pomodoro_store.exists() {
            let state: PersistentState = pomodoro_store.load().unwrap_or_else(|_| {
                // Try backward compatibility or default
                PersistentState {
                    pomodoro: Pomodoro::default(),
                    settings: AppSettings::default(),
                }
            });
            (state.pomodoro, state.settings)
        } else {
            (Pomodoro::default(), AppSettings::default())
        };

        let theme = crate::ui::theme::Theme::all().into_iter()
            .find(|t| t.name == settings.theme_name)
            .unwrap_or_default();
        
        let i18n = I18n::new(settings.language);
        
        let mut app = Self {
            tasks,
            selected_task_index: 0,
            input_mode: InputMode::Normal,
            active_panel: ActivePanel::TaskList,
            store,
            input_buffer: String::new(),
            show_help: false,
            wizard_title: String::new(),
            wizard_desc: String::new(),
            wizard_date: String::new(),
            pomodoro,
            task_list_state: ratatui::widgets::ListState::default(),
            pomodoro_store,
            theme,
            settings,
            i18n,
            menu_cursor: 0,
            detail_scroll_offset: 0,
            status_message: None,
            update_info: None,
            last_sync: std::time::Instant::now(),
            filter_text: String::new(),
            sort_mode: SortMode::Priority,
            frame_count: 0,
            onboarding_index: 0,
        };

        if app.settings.is_first_run {
            app.input_mode = InputMode::Onboarding;
        }

        for task in &mut app.tasks {
            if !task.review_subtasks.iter().any(|s| s.completed) && !task.custom_review_str.is_empty() {
                task.generate_review_subtasks();
            }
        }

        app.sort_tasks();
        Ok(app)
    }

    pub fn save(&self) -> Result<()> {
        self.store.save(&self.tasks)?;
        self.save_settings()
    }

    pub fn save_settings(&self) -> Result<()> {
        let state = PersistentState {
            pomodoro: self.pomodoro.clone(),
            settings: self.settings.clone(),
        };
        let content = serde_json::to_string_pretty(&state)?;
        let path = self.pomodoro_store.path();
        let tmp_path = path.with_extension("tmp");
        std::fs::write(&tmp_path, content)?;
        std::fs::rename(&tmp_path, path)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn trigger_notion_sync(&self, task_id: uuid::Uuid) {
        if self.settings.notion_api_key.is_none() || self.settings.notion_database_id.is_none() {
            return;
        }

        if let Some(task) = self.tasks.iter().find(|t| t.id == task_id).cloned() {
            let settings = self.settings.clone();
            // In a real app we'd need a way to update notion_id back, 
            // but for now we push updates in background.
            std::thread::spawn(move || {
                let _ = crate::integrations::notion::sync_task_to_notion(&settings, &task);
            });
        }
    }

    pub fn add_task_full(&mut self, title: String, description: String, date_str: String, custom_review: String) {
        let mut task = Task::new(title, description);
        
        task.due_date = crate::utils::parse_date_input(&date_str);
        
        task.custom_review_str = custom_review;
        task.generate_review_subtasks();
        let task_id = task.id;
        self.tasks.push(task);
        self.sort_tasks();
        
        // Select the newly added task
        if let Some(pos) = self.tasks.iter().position(|t| t.id == task_id) {
            self.selected_task_index = pos;
            self.task_list_state.select(Some(pos));
        }
        
        let _ = self.save();
    }

    pub fn import_tasks(&mut self, path: String) {
        match crate::utils::export::import_tasks_from_xlsx(&path) {
            Ok(new_tasks) => {
                let count = new_tasks.len();
                self.tasks.extend(new_tasks);
                self.sort_tasks();
                let _ = self.save();
                self.status_message = Some(format!("Imported {} tasks", count));
            }
            Err(e) => {
                self.status_message = Some(format!("Import error: {}", e));
            }
        }
    }

    pub fn sort_tasks(&mut self) {
        use chrono::Local;
        let today = Local::now().date_naive();
        
        match self.sort_mode {
            SortMode::Priority => {
                self.tasks.sort_by(|a, b| {
                    let cat_a = match a.due_date.or(a.review_state.next_review) {
                        Some(dt) => {
                            let diff = dt.with_timezone(&Local).date_naive().signed_duration_since(today).num_days();
                            if diff < 0 { 0 } else if diff == 0 { 1 } else { 2 }
                        },
                        None => 3,
                    };
                    let cat_b = match b.due_date.or(b.review_state.next_review) {
                        Some(dt) => {
                            let diff = dt.with_timezone(&Local).date_naive().signed_duration_since(today).num_days();
                            if diff < 0 { 0 } else if diff == 0 { 1 } else { 2 }
                        },
                        None => 3,
                    };
                    
                    if cat_a != cat_b {
                        return cat_a.cmp(&cat_b);
                    }
                    if a.importance != b.importance {
                        return b.importance.partial_cmp(&a.importance).unwrap_or(std::cmp::Ordering::Equal);
                    }
                    b.created_at.cmp(&a.created_at)
                });
            }
            SortMode::Date => {
                self.tasks.sort_by(|a, b| {
                    let d_a = a.due_date.or(a.review_state.next_review);
                    let d_b = b.due_date.or(b.review_state.next_review);
                    match (d_a, d_b) {
                        (Some(a), Some(b)) => a.cmp(&b),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => b.created_at.cmp(&a.created_at),
                    }
                });
            }
            SortMode::Title => {
                self.tasks.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
            }
        }
    }

    pub fn cycle_sort(&mut self) {
        self.sort_mode = match self.sort_mode {
            SortMode::Priority => SortMode::Date,
            SortMode::Date => SortMode::Title,
            SortMode::Title => SortMode::Priority,
        };
        self.sort_tasks();
    }

    pub fn filtered_tasks(&self) -> Vec<(usize, &Task)> {
        self.tasks.iter().enumerate()
            .filter(|(_, t)| {
                if self.filter_text.is_empty() { return true; }
                t.title.to_lowercase().contains(&self.filter_text.to_lowercase()) ||
                t.description.to_lowercase().contains(&self.filter_text.to_lowercase())
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn filtered_task_count(&self) -> usize {
        if self.filter_text.is_empty() {
            self.tasks.len()
        } else {
            self.filtered_tasks().len()
        }
    }

    pub fn current_task(&self) -> Option<&Task> {
        self.tasks.get(self.selected_task_index)
    }

    pub fn current_task_mut(&mut self) -> Option<&mut Task> {
        self.tasks.get_mut(self.selected_task_index)
    }

    pub fn pomodoro_tick(&mut self) -> bool {
        self.pomodoro.tick()
    }

    pub fn next_task(&mut self) {
        if !self.tasks.is_empty() {
            self.selected_task_index = (self.selected_task_index + 1) % self.tasks.len();
            self.task_list_state.select(Some(self.selected_task_index));
        }
    }

    pub fn previous_task(&mut self) {
        if !self.tasks.is_empty() {
            if self.selected_task_index > 0 {
                self.selected_task_index -= 1;
            } else {
                self.selected_task_index = self.tasks.len() - 1;
            }
            self.task_list_state.select(Some(self.selected_task_index));
        }
    }

    pub fn delete_task(&mut self) {
        if !self.tasks.is_empty() {
            self.tasks.remove(self.selected_task_index);
            if self.selected_task_index >= self.tasks.len() && !self.tasks.is_empty() {
                self.selected_task_index = self.tasks.len() - 1;
            }
            self.task_list_state.select(Some(self.selected_task_index));
            let _ = self.save();
        }
    }

    pub fn apply_theme(&mut self, theme_name: String) {
        if let Some(t) = crate::ui::theme::Theme::all().into_iter().find(|t| t.name == theme_name) {
            self.theme = t;
            self.settings.theme_name = theme_name;
            let _ = self.save_settings();
        }
    }

    pub fn set_language(&mut self, lang: Language) {
        self.settings.language = lang;
        self.i18n = I18n::new(lang);
        let _ = self.save_settings();
    }

    pub fn t<'a>(&self, key: &'a str) -> &'a str {
        self.i18n.t(key)
    }

    pub fn next_theme(&mut self) {
        let themes = crate::ui::theme::Theme::all();
        let current_index = themes.iter().position(|t| t.name == self.theme.name).unwrap_or(0);
        let next_index = (current_index + 1) % themes.len();
        self.apply_theme(themes[next_index].name.clone());
    }

    pub fn next_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::TaskList => ActivePanel::TaskDetail,
            ActivePanel::TaskDetail => ActivePanel::Pomodoro,
            ActivePanel::Pomodoro => ActivePanel::TaskList,
        };
    }

    pub fn previous_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::TaskList => ActivePanel::Pomodoro,
            ActivePanel::TaskDetail => ActivePanel::TaskList,
            ActivePanel::Pomodoro => ActivePanel::TaskDetail,
        };
    }

    pub fn toggle_task_completion(&mut self) {
        let mut changed = false;
        let mut should_delete = false;
        if let Some(task) = self.current_task_mut() {
            if !task.completed {
                task.completed = true;
                // DO NOT zero out due_date as it is needed to generate reviews based on original date
                task.generate_review_subtasks();
                if task.review_subtasks.is_empty() {
                    should_delete = true;
                }
            } else {
                if !task.review_subtasks.is_empty() && task.review_subtasks.iter().any(|s| !s.completed) {
                    task.complete_next_subtask();
                    if !task.review_subtasks.iter().any(|s| !s.completed) {
                        should_delete = true;
                    }
                } else {
                    task.completed = false; 
                }
            }
            changed = true;
        }
        if should_delete {
            self.delete_task();
        } else if changed {
            self.sort_tasks();
            let _ = self.save();
        }
    }

    pub fn untoggle_task_completion(&mut self) {
        let mut changed = false;
        if let Some(task) = self.current_task_mut() {
            if !task.review_subtasks.is_empty() && task.review_subtasks.iter().any(|s| s.completed) {
                if let Some(sub) = task.review_subtasks.iter_mut().rev().find(|s| s.completed) {
                    sub.completed = false;
                    changed = true;
                }
            } else if task.completed {
                task.completed = false;
                changed = true;
            }
        }
        if changed {
            self.sort_tasks();
            let _ = self.save();
        }
    }

    pub fn toggle_pomodoro(&mut self) {
        self.pomodoro.is_running = !self.pomodoro.is_running;
    }

    pub fn skip_pomodoro_phase(&mut self) {
        self.pomodoro.next_phase();
    }

    pub fn set_current_task_description(&mut self, desc: String) {
        let mut changed = false;
        if let Some(task) = self.current_task_mut() {
            task.description = desc;
            changed = true;
        }
        if changed {
            let _ = self.save();
        }
    }

    pub fn set_current_task_title(&mut self, title: String) {
        let mut changed = false;
        if let Some(task) = self.current_task_mut() {
            task.title = title;
            changed = true;
        }
        if changed {
            let _ = self.save();
        }
    }

    pub fn toggle_importance(&mut self) {
        let mut changed = false;
        if let Some(task) = self.current_task_mut() {
            task.importance = match task.importance {
                crate::model::task::Importance::Low => crate::model::task::Importance::Medium,
                crate::model::task::Importance::Medium => crate::model::task::Importance::High,
                crate::model::task::Importance::High => crate::model::task::Importance::Urgent,
                crate::model::task::Importance::Urgent => crate::model::task::Importance::Low,
            };
            changed = true;
        }
        if changed {
            let _ = self.save();
        }
    }

    pub fn next_pomodoro_profile(&mut self) {
        if self.pomodoro.is_running { return; }
        if !self.pomodoro.profiles.is_empty() {
            self.pomodoro.active_profile_index = (self.pomodoro.active_profile_index + 1) % self.pomodoro.profiles.len();
            self.pomodoro.reset();
        }
    }

    pub fn prev_pomodoro_profile(&mut self) {
        if self.pomodoro.is_running { return; }
        if !self.pomodoro.profiles.is_empty() {
            if self.pomodoro.active_profile_index == 0 {
                self.pomodoro.active_profile_index = self.pomodoro.profiles.len() - 1;
            } else {
                self.pomodoro.active_profile_index -= 1;
            }
            self.pomodoro.reset();
        }
    }

    pub fn edit_pomodoro_profile(&mut self, cfg: String) {
        let parts: Vec<&str> = cfg.split_whitespace().collect();
        if !parts.is_empty() {
            let name = parts[0];
            let work = parts.get(1).and_then(|p| p.parse::<u32>().ok()).unwrap_or(25);
            let short = parts.get(2).and_then(|p| p.parse::<u32>().ok()).unwrap_or(5);
            let long = parts.get(3).and_then(|p| p.parse::<u32>().ok()).unwrap_or(15);
            
            if let Some(profile) = self.pomodoro.profiles.get_mut(self.pomodoro.active_profile_index) {
                profile.name = name.to_string();
                profile.work_duration = work;
                profile.short_break = short;
                profile.long_break = long;
                if !self.pomodoro.is_running {
                    self.pomodoro.reset();
                }
            }
            let _ = self.save(); // Note: Pomodoro is implicitly saved or skipped depending on model, but we invoke save.
        }
    }

    pub fn set_custom_review(&mut self, review_str: String) {
        let mut changed = false;
        if let Some(task) = self.current_task_mut() {
            task.custom_review_str = review_str;
            task.generate_review_subtasks();
            changed = true;
        }
        if changed {
            self.sort_tasks();
            let _ = self.save();
        }
    }

    pub fn reset_pomodoro_timer(&mut self) {
        self.pomodoro.is_running = false;
        self.pomodoro.reset();
    }

    pub fn force_pomodoro_break(&mut self) {
        self.pomodoro.force_break();
    }

    pub fn go_to_first_task(&mut self) {
        if !self.tasks.is_empty() {
            self.selected_task_index = 0;
            self.task_list_state.select(Some(0));
        }
    }

    pub fn go_to_last_task(&mut self) {
        if !self.tasks.is_empty() {
            self.selected_task_index = self.tasks.len() - 1;
            self.task_list_state.select(Some(self.selected_task_index));
        }
    }

    pub fn scroll_detail_down(&mut self) {
        self.detail_scroll_offset = self.detail_scroll_offset.saturating_add(1);
    }

    pub fn scroll_detail_up(&mut self) {
        self.detail_scroll_offset = self.detail_scroll_offset.saturating_sub(1);
    }
}
