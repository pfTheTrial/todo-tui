use crate::i18n::{I18n, Language};
use crate::integrations::SyncProvider;
use crate::model::{
    settings::AppSettings, Pomodoro, SyncAction, SyncJob, SyncProviderMetrics, Task,
};
use crate::storage::JsonStore;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

const CURRENT_SCHEMA_VERSION: u32 = 1;

fn current_schema_version() -> u32 {
    CURRENT_SCHEMA_VERSION
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistentState {
    #[serde(default = "current_schema_version")]
    pub schema_version: u32,
    pub pomodoro: Pomodoro,
    pub settings: AppSettings,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SyncSummary {
    pub pushed: usize,
    pub deleted: usize,
    pub pulled: usize,
    pub skipped: usize,
    pub failed: usize,
    pub queued: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SyncQueueStatus {
    pub pending: usize,
    pub retrying: usize,
    pub failed: usize,
    pub pending_actions: SyncActionCounts,
    pub retrying_actions: SyncActionCounts,
    pub failed_actions: SyncActionCounts,
    pub next_retry_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SyncActionCounts {
    pub push: usize,
    pub delete: usize,
    pub pull: usize,
}

impl SyncActionCounts {
    fn increment(&mut self, action: SyncAction) {
        match action {
            SyncAction::Push => self.push += 1,
            SyncAction::Delete => self.delete += 1,
            SyncAction::Pull => self.pull += 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskSyncState {
    Pending,
    Failed,
    Synced,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskSyncStatus {
    pub provider: String,
    pub action: SyncAction,
    pub state: TaskSyncState,
    pub attempts: u32,
    pub next_retry_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct PersistentSyncState {
    #[serde(default)]
    queue: Vec<SyncJob>,
    #[serde(default)]
    metrics: Vec<SyncProviderMetrics>,
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
    Onboarding,
    ConfirmingUpdate,
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
    pub sync_queue_store: JsonStore,
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
    pub is_npm: bool,
    // Resource usage (sampled periodically, not every frame)
    pub sys_ram_mb: f32,
    pub sys_cpu_pct: f32,
    pub sync_queue: Vec<SyncJob>,
    pub sync_metrics: Vec<SyncProviderMetrics>,
    pub tasks_file_modified: Option<std::time::SystemTime>,
    _tasks_file_watcher: Option<notify::RecommendedWatcher>,
    tasks_file_events: Option<Receiver<()>>,
    last_tasks_reload_check: Instant,
}

impl App {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        let tasks_path = data_dir.join("tasks.json");
        let pomo_path = data_dir.join("pomodoro.json");
        let sync_queue_path = data_dir.join("sync_queue.json");

        let store = JsonStore::new(tasks_path);
        let tasks_file_modified = store.modified_time().unwrap_or(None);
        let (tasks_file_watcher, tasks_file_events) = match create_tasks_watcher(store.path()) {
            Ok(parts) => (Some(parts.0), Some(parts.1)),
            Err(err) => {
                eprintln!("watcher de tasks.json indisponível; usando fallback por polling: {err}");
                (None, None)
            }
        };
        let tasks: Vec<Task> = if store.exists() {
            match store.load() {
                Ok(tasks) => tasks,
                Err(err) => {
                    let backup = store.backup_corrupt()?;
                    eprintln!(
                        "tasks.json inválido foi preservado em {:?}; iniciando com lista vazia. Erro: {}",
                        backup, err
                    );
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        let pomodoro_store = JsonStore::new(pomo_path);
        let sync_queue_store = JsonStore::new(sync_queue_path);
        let (sync_queue, sync_metrics) = load_sync_state(&sync_queue_store);
        let (pomodoro, settings) = if pomodoro_store.exists() {
            match pomodoro_store.load::<PersistentState>() {
                Ok(state) => (state.pomodoro, state.settings),
                Err(err) => {
                    let backup = pomodoro_store.backup_corrupt()?;
                    eprintln!(
                        "pomodoro.json inválido foi preservado em {:?}; usando configurações padrão. Erro: {}",
                        backup, err
                    );
                    (Pomodoro::default(), AppSettings::default())
                }
            }
        } else {
            (Pomodoro::default(), AppSettings::default())
        };

        let theme = crate::ui::theme::Theme::all()
            .into_iter()
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
            sync_queue_store,
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
            is_npm: false, // Calculated after instantiation
            sys_ram_mb: 0.0,
            sys_cpu_pct: 0.0,
            sync_queue,
            sync_metrics,
            tasks_file_modified,
            _tasks_file_watcher: tasks_file_watcher,
            tasks_file_events,
            last_tasks_reload_check: Instant::now(),
        };

        if app.settings.is_first_run {
            app.input_mode = InputMode::Onboarding;
        }

        for task in &mut app.tasks {
            task.migrate_remote_ids();
            if !task.review_subtasks.iter().any(|s| s.completed)
                && !task.custom_review_str.is_empty()
            {
                task.generate_review_subtasks();
            }
        }

        app.sort_tasks();
        Ok(app)
    }

    pub fn save(&mut self) -> Result<()> {
        self.store.save(&self.tasks)?;
        self.tasks_file_modified = self.store.modified_time()?;
        self.save_settings()?;
        self.save_sync_queue()
    }

    pub fn save_settings(&self) -> Result<()> {
        let state = PersistentState {
            schema_version: CURRENT_SCHEMA_VERSION,
            pomodoro: self.pomodoro.clone(),
            settings: self.settings.clone(),
        };
        self.pomodoro_store.save(&state)?;
        Ok(())
    }

    pub fn save_sync_queue(&self) -> Result<()> {
        let state = PersistentSyncState {
            queue: self.sync_queue.clone(),
            metrics: self.sync_metrics.clone(),
        };
        self.sync_queue_store.save(&state)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn trigger_notion_sync(&mut self, task_id: uuid::Uuid) {
        if self.settings.notion_api_key.is_none() || self.settings.notion_database_id.is_none() {
            return;
        }

        if let Some(pos) = self.tasks.iter().position(|t| t.id == task_id) {
            let provider = crate::integrations::notion::NotionProvider::new(self.settings.clone());
            if let Ok(notion_id) = provider.push_task(&self.tasks[pos]) {
                self.tasks[pos].set_remote_id("notion", notion_id);
                let _ = self.save();
            }
        }
    }

    pub fn sync_all_notion(&mut self) -> std::result::Result<SyncSummary, String> {
        if self.settings.notion_api_key.is_none() || self.settings.notion_database_id.is_none() {
            return Err("Notion não está configurado.".to_string());
        }

        let provider = crate::integrations::notion::NotionProvider::new(self.settings.clone());
        provider.health_check()?;
        let jobs: Vec<_> = self
            .tasks
            .iter()
            .map(|task| {
                (
                    task.id,
                    if task.is_deleted() {
                        SyncAction::Delete
                    } else {
                        SyncAction::Push
                    },
                )
            })
            .collect();
        for (task_id, action) in jobs {
            self.enqueue_sync_job("notion", task_id, action);
        }
        let summary = self.process_sync_queue(&provider)?;
        self.compact_synced_tombstones();
        self.save().map_err(|err| err.to_string())?;
        Ok(summary)
    }

    fn enqueue_sync_job(&mut self, provider: &str, task_id: uuid::Uuid, action: SyncAction) {
        let exists = self.sync_queue.iter().any(|job| {
            job.provider == provider && job.task_id == Some(task_id) && job.action == action
        });
        if !exists {
            self.sync_queue
                .push(SyncJob::new(provider.to_string(), task_id, action));
        }
    }

    fn enqueue_provider_job(&mut self, provider: &str, action: SyncAction) {
        let exists = self
            .sync_queue
            .iter()
            .any(|job| job.provider == provider && job.task_id.is_none() && job.action == action);
        if !exists {
            self.sync_queue
                .push(SyncJob::provider_job(provider.to_string(), action));
        }
    }

    pub fn process_sync_queue<P: SyncProvider>(
        &mut self,
        provider: &P,
    ) -> std::result::Result<SyncSummary, String> {
        let now = chrono::Utc::now();
        let mut summary = SyncSummary {
            queued: self.sync_queue.len(),
            ..SyncSummary::default()
        };
        let provider_name = provider.name();
        let mut remaining = Vec::new();
        let jobs = std::mem::take(&mut self.sync_queue);

        for mut job in jobs {
            if job.provider != provider_name || !job.is_due(now) {
                remaining.push(job);
                continue;
            }

            let result = match job.action {
                SyncAction::Pull => provider.pull_tasks().map(|pulled| {
                    summary.pulled += self.apply_pulled_tasks(provider_name, pulled);
                }),
                SyncAction::Push | SyncAction::Delete => {
                    let Some(task_id) = job.task_id else {
                        summary.skipped += 1;
                        continue;
                    };
                    let Some(task_pos) = self.tasks.iter().position(|task| task.id == task_id)
                    else {
                        summary.skipped += 1;
                        continue;
                    };

                    match job.action {
                        SyncAction::Push => {
                            provider.push_task(&self.tasks[task_pos]).map(|remote_id| {
                                self.tasks[task_pos].set_remote_id(provider_name, remote_id);
                                summary.pushed += 1;
                            })
                        }
                        SyncAction::Delete => {
                            if let Some(remote_id) = self.tasks[task_pos].remote_id(provider_name) {
                                provider.delete_task(remote_id).map(|()| {
                                    summary.deleted += 1;
                                })
                            } else {
                                summary.skipped += 1;
                                Ok(())
                            }
                        }
                        SyncAction::Pull => unreachable!(),
                    }
                }
            };

            if let Err(err) = result {
                self.record_sync_error(provider_name, job.action, &err, now);
                job.record_failure(err, now);
                summary.failed += 1;
                remaining.push(job);
            } else {
                self.record_sync_success(provider_name, now);
            }
        }

        self.sync_queue = remaining;
        Ok(summary)
    }

    pub fn retry_all_sync_jobs_now(&mut self, provider: Option<&str>) {
        for job in &mut self.sync_queue {
            if provider.is_none_or(|name| job.provider == name) {
                job.next_retry_at = None;
            }
        }
    }

    pub fn pull_from_notion(&mut self) -> std::result::Result<usize, String> {
        let provider = crate::integrations::notion::NotionProvider::new(self.settings.clone());
        provider.health_check()?;
        self.enqueue_provider_job("notion", SyncAction::Pull);
        let summary = self.process_sync_queue(&provider)?;
        if summary.pulled > 0 {
            self.sort_tasks();
        }
        self.save().map_err(|err| err.to_string())?;
        Ok(summary.pulled)
    }

    fn apply_pulled_tasks(&mut self, provider: &str, pulled: Vec<Task>) -> usize {
        let mut changed = 0;

        for remote_task in pulled {
            let remote_id = remote_task.remote_id(provider).map(str::to_string);
            let Some(remote_id) = remote_id else {
                continue;
            };

            if let Some(existing) = self
                .tasks
                .iter_mut()
                .find(|task| task.remote_id(provider) == Some(remote_id.as_str()))
            {
                if should_apply_remote(existing, &remote_task) {
                    existing.title = remote_task.title;
                    existing.description = remote_task.description;
                    existing.completed = remote_task.completed;
                    existing.importance = remote_task.importance;
                    existing.updated_at = remote_task.updated_at;
                    changed += 1;
                }
            } else {
                self.tasks.push(remote_task);
                changed += 1;
            }
        }

        if changed > 0 {
            self.sort_tasks();
        }

        changed
    }

    pub fn add_task_full(
        &mut self,
        title: String,
        description: String,
        date_str: String,
        custom_review: String,
    ) {
        let mut task = Task::new(title, description);

        task.due_date = crate::utils::parse_date_input(&date_str);

        task.custom_review_str = custom_review;
        task.generate_review_subtasks();
        task.mark_updated();
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

    #[allow(dead_code)]
    pub fn import_tasks(&mut self, path: String) {
        match crate::utils::export::import_tasks_from_xlsx(&path) {
            Ok(mut new_tasks) => {
                let count = new_tasks.len();
                for task in &mut new_tasks {
                    task.mark_updated();
                }
                self.tasks.extend(new_tasks);
                self.sort_tasks();
                let _ = self.save();
                self.status_message = Some(format!(
                    "{} {} {}",
                    "✅",
                    count,
                    self.i18n.t("msg.imported")
                ));
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
                    let cat_a = match a.effective_date() {
                        Some(dt) => {
                            let diff = dt
                                .with_timezone(&Local)
                                .date_naive()
                                .signed_duration_since(today)
                                .num_days();
                            if diff < 0 {
                                0
                            } else if diff == 0 {
                                1
                            } else {
                                2
                            }
                        }
                        None => 3,
                    };
                    let cat_b = match b.effective_date() {
                        Some(dt) => {
                            let diff = dt
                                .with_timezone(&Local)
                                .date_naive()
                                .signed_duration_since(today)
                                .num_days();
                            if diff < 0 {
                                0
                            } else if diff == 0 {
                                1
                            } else {
                                2
                            }
                        }
                        None => 3,
                    };

                    if cat_a != cat_b {
                        return cat_a.cmp(&cat_b);
                    }
                    if a.importance != b.importance {
                        return b
                            .importance
                            .partial_cmp(&a.importance)
                            .unwrap_or(std::cmp::Ordering::Equal);
                    }
                    b.created_at.cmp(&a.created_at)
                });
            }
            SortMode::Date => {
                self.tasks.sort_by(|a, b| {
                    let d_a = a.effective_date();
                    let d_b = b.effective_date();
                    match (d_a, d_b) {
                        (Some(a), Some(b)) => a.cmp(&b),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => b.created_at.cmp(&a.created_at),
                    }
                });
            }
            SortMode::Title => {
                self.tasks
                    .sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
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
        if self.filter_text.is_empty() {
            return self
                .tasks
                .iter()
                .enumerate()
                .filter(|(_, task)| !task.is_deleted())
                .collect();
        }
        let filter = self.filter_text.to_lowercase();
        self.tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| !task.is_deleted())
            .filter(|(_, t)| {
                t.title.to_lowercase().contains(&filter)
                    || t.description.to_lowercase().contains(&filter)
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn filtered_task_count(&self) -> usize {
        if self.filter_text.is_empty() {
            self.tasks.iter().filter(|task| !task.is_deleted()).count()
        } else {
            self.filtered_tasks().len()
        }
    }

    pub fn current_task(&self) -> Option<&Task> {
        self.tasks
            .get(self.selected_task_index)
            .filter(|task| !task.is_deleted())
    }

    pub fn current_task_mut(&mut self) -> Option<&mut Task> {
        self.tasks
            .get_mut(self.selected_task_index)
            .filter(|task| !task.is_deleted())
    }

    pub fn pomodoro_tick(&mut self) -> bool {
        self.pomodoro.tick()
    }

    pub fn next_task(&mut self) {
        let visible = self.visible_task_indices();
        if !visible.is_empty() {
            let pos = visible
                .iter()
                .position(|idx| *idx == self.selected_task_index)
                .unwrap_or(0);
            self.selected_task_index = visible[(pos + 1) % visible.len()];
            self.task_list_state.select(Some(self.selected_task_index));
        }
    }

    pub fn previous_task(&mut self) {
        let visible = self.visible_task_indices();
        if !visible.is_empty() {
            let pos = visible
                .iter()
                .position(|idx| *idx == self.selected_task_index)
                .unwrap_or(0);
            self.selected_task_index = if pos > 0 {
                visible[pos - 1]
            } else {
                *visible.last().unwrap_or(&0)
            };
            self.task_list_state.select(Some(self.selected_task_index));
        }
    }

    pub fn delete_task(&mut self) {
        if let Some(task) = self.current_task_mut() {
            task.mark_deleted();
            let visible = self.visible_task_indices();
            if let Some(next) = visible
                .into_iter()
                .find(|idx| *idx != self.selected_task_index)
            {
                self.selected_task_index = next;
                self.task_list_state.select(Some(self.selected_task_index));
            } else {
                self.task_list_state.select(None);
            }
            let _ = self.save();
        }
    }

    pub fn apply_theme(&mut self, theme_name: String) {
        if let Some(t) = crate::ui::theme::Theme::all()
            .into_iter()
            .find(|t| t.name == theme_name)
        {
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
        let current_index = themes
            .iter()
            .position(|t| t.name == self.theme.name)
            .unwrap_or(0);
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
                if !task.review_subtasks.is_empty()
                    && task.review_subtasks.iter().any(|s| !s.completed)
                {
                    task.complete_next_subtask();
                    if !task.review_subtasks.iter().any(|s| !s.completed) {
                        should_delete = true;
                    }
                } else {
                    task.completed = false;
                }
            }
            task.mark_updated();
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
            if !task.review_subtasks.is_empty() && task.review_subtasks.iter().any(|s| s.completed)
            {
                if let Some(sub) = task.review_subtasks.iter_mut().rev().find(|s| s.completed) {
                    sub.completed = false;
                    task.mark_updated();
                    changed = true;
                }
            } else if task.completed {
                task.completed = false;
                task.mark_updated();
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
            task.mark_updated();
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
            task.mark_updated();
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
            task.mark_updated();
            changed = true;
        }
        if changed {
            let _ = self.save();
        }
    }

    pub fn next_pomodoro_profile(&mut self) {
        if self.pomodoro.is_running {
            return;
        }
        if !self.pomodoro.profiles.is_empty() {
            self.pomodoro.active_profile_index =
                (self.pomodoro.active_profile_index + 1) % self.pomodoro.profiles.len();
            self.pomodoro.reset();
        }
    }

    pub fn prev_pomodoro_profile(&mut self) {
        if self.pomodoro.is_running {
            return;
        }
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
            let work = parts
                .get(1)
                .and_then(|p| p.parse::<u32>().ok())
                .unwrap_or(25);
            let short = parts
                .get(2)
                .and_then(|p| p.parse::<u32>().ok())
                .unwrap_or(5);
            let long = parts
                .get(3)
                .and_then(|p| p.parse::<u32>().ok())
                .unwrap_or(15);

            if let Some(profile) = self
                .pomodoro
                .profiles
                .get_mut(self.pomodoro.active_profile_index)
            {
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
            task.mark_updated();
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
        if let Some(first) = self.visible_task_indices().first().copied() {
            self.selected_task_index = first;
            self.task_list_state.select(Some(first));
        }
    }

    pub fn go_to_last_task(&mut self) {
        if let Some(last) = self.visible_task_indices().last().copied() {
            self.selected_task_index = last;
            self.task_list_state.select(Some(last));
        }
    }

    pub fn scroll_detail_down(&mut self) {
        self.detail_scroll_offset = self.detail_scroll_offset.saturating_add(1);
    }

    pub fn scroll_detail_up(&mut self) {
        self.detail_scroll_offset = self.detail_scroll_offset.saturating_sub(1);
    }

    fn visible_task_indices(&self) -> Vec<usize> {
        self.tasks
            .iter()
            .enumerate()
            .filter_map(|(idx, task)| (!task.is_deleted()).then_some(idx))
            .collect()
    }

    pub fn reload_tasks_if_changed(&mut self) -> Result<bool> {
        let modified = self.store.modified_time()?;
        if modified.is_none() || modified == self.tasks_file_modified {
            return Ok(false);
        }

        let mut tasks: Vec<Task> = self.store.load()?;
        for task in &mut tasks {
            task.migrate_remote_ids();
        }
        self.tasks = tasks;
        self.tasks_file_modified = modified;
        self.sort_tasks();
        Ok(true)
    }

    pub fn compact_synced_tombstones(&mut self) -> usize {
        let before = self.tasks.len();
        self.tasks.retain(|task| {
            !(task.is_deleted()
                && task.remote_id("notion").is_some()
                && !self
                    .sync_queue
                    .iter()
                    .any(|job| job.task_id == Some(task.id)))
        });
        before.saturating_sub(self.tasks.len())
    }

    pub fn sync_queue_status(&self, provider: Option<&str>) -> SyncQueueStatus {
        let now = chrono::Utc::now();
        let mut status = SyncQueueStatus::default();
        for job in &self.sync_queue {
            if provider.is_some_and(|name| job.provider != name) {
                continue;
            }
            if job.next_retry_at.is_some() {
                status.retrying += 1;
                status.retrying_actions.increment(job.action);
            } else {
                status.pending += 1;
                status.pending_actions.increment(job.action);
            }
            if job.last_error.is_some() {
                status.failed += 1;
                status.failed_actions.increment(job.action);
            }
            if let Some(next_retry_at) = job.next_retry_at {
                if next_retry_at > now
                    && status
                        .next_retry_at
                        .is_none_or(|current| next_retry_at < current)
                {
                    status.next_retry_at = Some(next_retry_at);
                }
            }
        }
        status
    }

    pub fn sync_metrics(&self, provider: &str) -> Option<&SyncProviderMetrics> {
        self.sync_metrics
            .iter()
            .find(|metrics| metrics.provider == provider)
    }

    pub fn task_sync_status(&self, task_id: uuid::Uuid, provider: &str) -> Option<TaskSyncStatus> {
        let mut best: Option<(u8, TaskSyncStatus)> = None;

        for job in self
            .sync_queue
            .iter()
            .filter(|job| job.provider == provider && job.task_id == Some(task_id))
        {
            let state = if job.last_error.is_some() {
                TaskSyncState::Failed
            } else {
                TaskSyncState::Pending
            };
            let rank = match state {
                TaskSyncState::Failed => 3,
                TaskSyncState::Pending => 2,
                TaskSyncState::Synced => 1,
            };
            let status = TaskSyncStatus {
                provider: job.provider.clone(),
                action: job.action,
                state,
                attempts: job.attempts,
                next_retry_at: job.next_retry_at,
                last_error: job.last_error.clone(),
            };
            if best.as_ref().is_none_or(|(current, _)| rank > *current) {
                best = Some((rank, status));
            }
        }

        if let Some((_, status)) = best {
            return Some(status);
        }

        self.tasks
            .iter()
            .find(|task| task.id == task_id)
            .and_then(|task| task.remote_id(provider))
            .map(|_| TaskSyncStatus {
                provider: provider.to_string(),
                action: SyncAction::Push,
                state: TaskSyncState::Synced,
                attempts: 0,
                next_retry_at: None,
                last_error: None,
            })
    }

    pub fn reload_tasks_if_external_change(&mut self) -> Result<bool> {
        let watch_event = self.take_pending_task_file_event();
        let fallback_due = self.last_tasks_reload_check.elapsed() >= Duration::from_secs(30);
        if !watch_event && !fallback_due {
            return Ok(false);
        }

        self.last_tasks_reload_check = Instant::now();
        self.reload_tasks_if_changed()
    }

    fn take_pending_task_file_event(&mut self) -> bool {
        let Some(receiver) = &self.tasks_file_events else {
            return false;
        };

        let mut changed = false;
        while receiver.try_recv().is_ok() {
            changed = true;
        }
        changed
    }

    fn record_sync_success(&mut self, provider: &str, now: chrono::DateTime<chrono::Utc>) {
        let metrics = self.ensure_sync_metrics(provider);
        metrics.record_success(now);
    }

    fn record_sync_error(
        &mut self,
        provider: &str,
        action: SyncAction,
        error: &str,
        now: chrono::DateTime<chrono::Utc>,
    ) {
        let metrics = self.ensure_sync_metrics(provider);
        metrics.record_error(action, error, now);
    }

    fn ensure_sync_metrics(&mut self, provider: &str) -> &mut SyncProviderMetrics {
        if let Some(pos) = self
            .sync_metrics
            .iter()
            .position(|metrics| metrics.provider == provider)
        {
            return &mut self.sync_metrics[pos];
        }
        self.sync_metrics
            .push(SyncProviderMetrics::new(provider.to_string()));
        self.sync_metrics
            .last_mut()
            .expect("sync metrics just inserted")
    }
}

fn should_apply_remote(local: &Task, remote: &Task) -> bool {
    if local.is_deleted() {
        return false;
    }
    remote.updated_at > local.updated_at
}

fn create_tasks_watcher(path: &Path) -> notify::Result<(notify::RecommendedWatcher, Receiver<()>)> {
    use notify::{EventKind, RecursiveMode, Watcher};

    let (tx, rx) = std::sync::mpsc::channel();
    let watch_dir = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let watch_name = path.file_name().map(|name| name.to_os_string());

    let mut watcher = notify::recommended_watcher(move |result: notify::Result<notify::Event>| {
        let Ok(event) = result else {
            return;
        };

        let is_relevant_path = watch_name.as_ref().is_none_or(|name| {
            event
                .paths
                .iter()
                .any(|event_path| event_path.file_name() == Some(name.as_os_str()))
        });
        let is_relevant_kind = matches!(
            event.kind,
            EventKind::Create(_)
                | EventKind::Modify(_)
                | EventKind::Remove(_)
                | EventKind::Any
                | EventKind::Other
        );

        if is_relevant_path && is_relevant_kind {
            let _ = tx.send(());
        }
    })?;

    watcher.watch(&watch_dir, RecursiveMode::NonRecursive)?;
    Ok((watcher, rx))
}

fn load_sync_state(store: &JsonStore) -> (Vec<SyncJob>, Vec<SyncProviderMetrics>) {
    if !store.exists() {
        return (Vec::new(), Vec::new());
    }

    if let Ok(state) = store.load::<PersistentSyncState>() {
        return (state.queue, state.metrics);
    }

    if let Ok(queue) = store.load::<Vec<SyncJob>>() {
        return (queue, Vec::new());
    }

    (Vec::new(), Vec::new())
}

#[cfg(test)]
mod tests {
    use super::{load_sync_state, should_apply_remote, App};
    use crate::integrations::SyncProvider;
    use crate::model::{SyncAction, SyncJob, SyncProviderMetrics, Task};
    use crate::storage::JsonStore;
    use std::{sync::Mutex, time::Duration};
    use uuid::Uuid;

    struct FakeProvider {
        fail: bool,
        deleted: Mutex<Vec<String>>,
        pulled: Vec<Task>,
    }

    impl FakeProvider {
        fn ok() -> Self {
            Self {
                fail: false,
                deleted: Mutex::new(Vec::new()),
                pulled: Vec::new(),
            }
        }

        fn failing() -> Self {
            Self {
                fail: true,
                deleted: Mutex::new(Vec::new()),
                pulled: Vec::new(),
            }
        }

        fn with_pull(tasks: Vec<Task>) -> Self {
            Self {
                fail: false,
                deleted: Mutex::new(Vec::new()),
                pulled: tasks,
            }
        }
    }

    impl SyncProvider for FakeProvider {
        fn name(&self) -> &'static str {
            "fake"
        }

        fn health_check(&self) -> Result<(), String> {
            Ok(())
        }

        fn push_task(&self, task: &Task) -> Result<String, String> {
            if self.fail {
                Err("push failed".to_string())
            } else {
                Ok(format!("remote-{}", task.id))
            }
        }

        fn delete_task(&self, remote_id: &str) -> Result<(), String> {
            if self.fail {
                Err("delete failed".to_string())
            } else {
                self.deleted.lock().unwrap().push(remote_id.to_string());
                Ok(())
            }
        }

        fn pull_tasks(&self) -> Result<Vec<Task>, String> {
            Ok(self.pulled.clone())
        }
    }

    #[test]
    fn delete_task_is_soft_delete_and_hides_from_filtered_tasks() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        app.add_task_full("B".to_string(), String::new(), String::new(), String::new());
        app.selected_task_index = 0;

        app.delete_task();

        assert_eq!(app.tasks.len(), 2);
        assert_eq!(app.filtered_task_count(), 1);
        assert!(app.tasks.iter().any(|task| task.deleted_at.is_some()));
    }

    #[test]
    fn conflict_resolution_prefers_newer_remote_only() {
        let mut local = Task::new("Local".to_string(), String::new());
        let mut remote = Task::new("Remote".to_string(), String::new());
        remote.updated_at = local.updated_at + chrono::Duration::seconds(1);

        assert!(should_apply_remote(&local, &remote));

        local.mark_deleted();
        assert!(!should_apply_remote(&local, &remote));
    }

    #[test]
    fn process_sync_queue_sets_remote_id_on_success() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        let task_id = app.tasks[0].id;
        app.sync_queue
            .push(SyncJob::new("fake", task_id, SyncAction::Push));

        let summary = app.process_sync_queue(&FakeProvider::ok()).unwrap();

        assert_eq!(summary.pushed, 1);
        assert!(app.sync_queue.is_empty());
        assert!(app.tasks[0].remote_id("fake").is_some());
    }

    #[test]
    fn process_sync_queue_keeps_failed_job_with_retry() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        let task_id = app.tasks[0].id;
        app.sync_queue
            .push(SyncJob::new("fake", task_id, SyncAction::Push));

        let summary = app.process_sync_queue(&FakeProvider::failing()).unwrap();

        assert_eq!(summary.failed, 1);
        assert_eq!(app.sync_queue.len(), 1);
        assert_eq!(app.sync_queue[0].attempts, 1);
        assert!(app.sync_queue[0].next_retry_at.is_some());
    }

    #[test]
    fn process_sync_queue_deletes_remote_tombstones() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        let task_id = app.tasks[0].id;
        app.tasks[0].set_remote_id("fake", "remote-a");
        app.tasks[0].mark_deleted();
        app.sync_queue
            .push(SyncJob::new("fake", task_id, SyncAction::Delete));
        let provider = FakeProvider::ok();

        let summary = app.process_sync_queue(&provider).unwrap();

        assert_eq!(summary.deleted, 1);
        assert_eq!(provider.deleted.lock().unwrap().as_slice(), ["remote-a"]);
    }

    #[test]
    fn reload_tasks_if_changed_loads_external_file() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full(
            "Before".to_string(),
            String::new(),
            String::new(),
            String::new(),
        );
        let external = vec![Task::new("External".to_string(), String::new())];
        app.store.save(&external).unwrap();
        app.tasks_file_modified = None;

        assert!(app.reload_tasks_if_changed().unwrap());
        assert_eq!(app.tasks[0].title, "External");
    }

    #[test]
    fn skipped_queue_job_for_missing_task_is_removed() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.sync_queue
            .push(SyncJob::new("fake", Uuid::new_v4(), SyncAction::Push));

        let summary = app.process_sync_queue(&FakeProvider::ok()).unwrap();

        assert_eq!(summary.skipped, 1);
        assert!(app.sync_queue.is_empty());
    }

    #[test]
    fn retry_all_sync_jobs_now_clears_retry_deadline() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        let mut job = SyncJob::new("notion", Uuid::new_v4(), SyncAction::Push);
        job.record_failure("fail", chrono::Utc::now());
        app.sync_queue.push(job);

        app.retry_all_sync_jobs_now(Some("notion"));

        assert!(app.sync_queue[0].next_retry_at.is_none());
    }

    #[test]
    fn compact_synced_tombstones_removes_deleted_synced_tasks() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        app.tasks[0].set_remote_id("notion", "page-a");
        app.tasks[0].mark_deleted();

        let removed = app.compact_synced_tombstones();

        assert_eq!(removed, 1);
        assert!(app.tasks.is_empty());
    }

    #[test]
    fn compact_synced_tombstones_keeps_deleted_tasks_with_pending_job() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        let task_id = app.tasks[0].id;
        app.tasks[0].set_remote_id("notion", "page-a");
        app.tasks[0].mark_deleted();
        app.sync_queue
            .push(SyncJob::new("notion", task_id, SyncAction::Delete));

        let removed = app.compact_synced_tombstones();

        assert_eq!(removed, 0);
        assert_eq!(app.tasks.len(), 1);
    }

    #[test]
    fn process_sync_queue_applies_pull_jobs() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        let mut remote = Task::new("Remote".to_string(), "Body".to_string());
        remote.set_remote_id("fake", "page-1");
        app.sync_queue
            .push(SyncJob::provider_job("fake", SyncAction::Pull));

        let summary = app
            .process_sync_queue(&FakeProvider::with_pull(vec![remote]))
            .unwrap();

        assert_eq!(summary.pulled, 1);
        assert_eq!(app.tasks.len(), 1);
        assert_eq!(app.tasks[0].title, "Remote");
    }

    #[test]
    fn process_sync_queue_records_provider_metrics() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        let task_id = app.tasks[0].id;
        app.sync_queue
            .push(SyncJob::new("fake", task_id, SyncAction::Push));

        app.process_sync_queue(&FakeProvider::ok()).unwrap();

        let metrics = app.sync_metrics("fake").expect("metrics");
        assert!(metrics.last_success_at.is_some());
        assert!(metrics.last_error.is_none());
    }

    #[test]
    fn process_sync_queue_records_recent_provider_errors() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        let task_id = app.tasks[0].id;
        app.sync_queue
            .push(SyncJob::new("fake", task_id, SyncAction::Push));

        app.process_sync_queue(&FakeProvider::failing()).unwrap();

        let metrics = app.sync_metrics("fake").expect("metrics");
        assert_eq!(metrics.last_error.as_deref(), Some("push failed"));
        assert_eq!(metrics.recent_errors.len(), 1);
        assert_eq!(metrics.recent_errors[0].action, SyncAction::Push);
    }

    #[test]
    fn task_sync_status_reports_failed_job_details() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        let task_id = app.tasks[0].id;
        let mut job = SyncJob::new("notion", task_id, SyncAction::Push);
        job.record_failure("timeout", chrono::Utc::now());
        app.sync_queue.push(job);

        let status = app
            .task_sync_status(task_id, "notion")
            .expect("sync status");

        assert_eq!(status.state, super::TaskSyncState::Failed);
        assert_eq!(status.last_error.as_deref(), Some("timeout"));
    }

    #[test]
    fn task_sync_status_reports_synced_remote_task() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full("A".to_string(), String::new(), String::new(), String::new());
        let task_id = app.tasks[0].id;
        app.tasks[0].set_remote_id("notion", "page-1");

        let status = app
            .task_sync_status(task_id, "notion")
            .expect("sync status");

        assert_eq!(status.state, super::TaskSyncState::Synced);
    }

    #[test]
    fn sync_queue_status_counts_actions_separately() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        let push = Uuid::new_v4();
        let delete = Uuid::new_v4();
        app.sync_queue
            .push(SyncJob::new("notion", push, SyncAction::Push));
        app.sync_queue
            .push(SyncJob::new("notion", delete, SyncAction::Delete));
        app.sync_queue
            .push(SyncJob::provider_job("notion", SyncAction::Pull));

        let status = app.sync_queue_status(Some("notion"));

        assert_eq!(status.pending_actions.push, 1);
        assert_eq!(status.pending_actions.delete, 1);
        assert_eq!(status.pending_actions.pull, 1);
    }

    #[test]
    fn reload_tasks_if_external_change_uses_watcher_events() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = App::new(dir.path().to_path_buf()).unwrap();
        app.add_task_full(
            "Local".to_string(),
            "before".to_string(),
            String::new(),
            String::new(),
        );
        let mut external = app.tasks.clone();
        external[0].title = "Externa".to_string();
        std::fs::write(
            dir.path().join("tasks.json"),
            serde_json::to_string_pretty(&external).unwrap(),
        )
        .unwrap();

        let mut reloaded = false;
        for _ in 0..30 {
            if app.reload_tasks_if_external_change().unwrap() {
                reloaded = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        assert!(reloaded);
        assert_eq!(app.tasks[0].title, "Externa");
    }

    #[test]
    fn load_sync_state_supports_new_persistent_format() {
        let dir = tempfile::tempdir().unwrap();
        let store = JsonStore::new(dir.path().join("sync_queue.json"));
        let state = super::PersistentSyncState {
            queue: vec![SyncJob::provider_job("notion", SyncAction::Pull)],
            metrics: vec![SyncProviderMetrics::new("notion")],
        };

        store.save(&state).unwrap();
        let (queue, metrics) = load_sync_state(&store);

        assert_eq!(queue.len(), 1);
        assert_eq!(metrics.len(), 1);
    }
}
