use super::review::{ReviewConfig, ReviewState, ReviewSubtask};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub enum Importance {
    Low,
    #[default]
    Medium,
    High,
    Urgent,
}

fn default_created_at() -> DateTime<Utc> {
    Utc::now()
}

fn default_updated_at() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    #[serde(default)]
    pub importance: Importance,
    #[serde(default = "default_created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "default_updated_at")]
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub due_date: Option<DateTime<Utc>>,
    pub review_state: ReviewState,
    #[serde(default)]
    pub review_subtasks: Vec<ReviewSubtask>,
    #[serde(default)]
    pub custom_review_str: String,
    #[serde(default)]
    pub remote_ids: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notion_id: Option<String>,
}

impl Task {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            importance: Importance::Medium,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
            due_date: None,
            review_state: ReviewState::default(),
            review_subtasks: Vec::new(),
            custom_review_str: String::new(),
            remote_ids: BTreeMap::new(),
            notion_id: None,
        }
    }

    pub fn migrate_remote_ids(&mut self) {
        if let Some(id) = self.notion_id.take() {
            self.remote_ids.entry("notion".to_string()).or_insert(id);
        }
    }

    pub fn remote_id(&self, provider: &str) -> Option<&str> {
        self.remote_ids.get(provider).map(String::as_str)
    }

    pub fn set_remote_id(&mut self, provider: impl Into<String>, id: impl Into<String>) {
        self.remote_ids.insert(provider.into(), id.into());
        if self.remote_ids.contains_key("notion") {
            self.notion_id = None;
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn mark_updated(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn mark_deleted(&mut self) {
        let now = Utc::now();
        self.deleted_at = Some(now);
        self.updated_at = now;
    }

    pub fn effective_date(&self) -> Option<DateTime<Utc>> {
        if self.completed && !self.review_subtasks.is_empty() {
            // Se está concluída mas tem sub-tarefas de revisão ativas, usa a data da review
            if let Some(s) = self.review_subtasks.iter().find(|s| !s.completed) {
                return Some(s.date);
            }
        }
        self.due_date.or(self.review_state.next_review)
    }

    pub fn generate_review_subtasks(&mut self) {
        if self.review_subtasks.iter().any(|s| s.completed) {
            return;
        }
        self.review_subtasks.clear();
        let parts: Vec<&str> = self.custom_review_str.split_whitespace().collect();
        let base_date = self.due_date.unwrap_or(self.created_at);
        for (i, p) in parts.iter().enumerate() {
            if let Some(date) = crate::utils::parse_relative_date(p, base_date) {
                self.review_subtasks.push(ReviewSubtask {
                    id: Uuid::new_v4(),
                    label: format!("Review {} ({})", i + 1, p),
                    date,
                    completed: false,
                });
            }
        }
        self.update_review_state_from_subtasks();
    }

    pub fn update_review_state_from_subtasks(&mut self) {
        if self.review_subtasks.is_empty() {
            return;
        }

        let pending = self.review_subtasks.iter().find(|s| !s.completed);
        self.review_state.next_review = pending.map(|s| s.date);
    }

    pub fn complete_next_subtask(&mut self) {
        if let Some(sub) = self.review_subtasks.iter_mut().find(|s| !s.completed) {
            sub.completed = true;
            self.update_review_state_from_subtasks();
        } else {
            self.complete_review(&ReviewConfig::default());
        }
    }

    pub fn complete_review(&mut self, config: &ReviewConfig) {
        self.review_state.advance(config);
    }
}

#[cfg(test)]
mod tests {
    use super::Task;
    use chrono::{TimeZone, Utc};

    #[test]
    fn generates_review_subtasks_from_custom_plan() {
        let mut task = Task::new("Study".to_string(), String::new());
        let base = Utc.with_ymd_and_hms(2026, 4, 21, 12, 0, 0).unwrap();
        task.due_date = Some(base);
        task.custom_review_str = "1d 1s 1m".to_string();

        task.generate_review_subtasks();

        assert_eq!(task.review_subtasks.len(), 3);
        assert_eq!(
            task.review_state.next_review,
            Some(base + chrono::Duration::days(1))
        );
    }

    #[test]
    fn completing_next_subtask_advances_next_review() {
        let mut task = Task::new("Study".to_string(), String::new());
        let base = Utc.with_ymd_and_hms(2026, 4, 21, 12, 0, 0).unwrap();
        task.due_date = Some(base);
        task.custom_review_str = "1d 2d".to_string();
        task.generate_review_subtasks();

        task.complete_next_subtask();

        assert!(task.review_subtasks[0].completed);
        assert_eq!(
            task.review_state.next_review,
            Some(base + chrono::Duration::days(2))
        );
    }

    #[test]
    fn migrates_legacy_notion_id_to_remote_ids() {
        let mut task = Task::new("Legacy".to_string(), String::new());
        task.notion_id = Some("page-123".to_string());

        task.migrate_remote_ids();

        assert_eq!(task.remote_id("notion"), Some("page-123"));
        assert_eq!(task.notion_id, None);
    }

    #[test]
    fn deserializes_legacy_notion_id() {
        let json = r#"{
            "id": "11111111-1111-4111-8111-111111111111",
            "title": "Legacy",
            "description": "",
            "completed": false,
            "review_state": {
                "current_step": 0,
                "last_review": null,
                "next_review": null,
                "completed_count": 0
            },
            "notion_id": "page-abc"
        }"#;

        let mut task: Task = serde_json::from_str(json).unwrap();
        task.migrate_remote_ids();

        assert_eq!(task.remote_id("notion"), Some("page-abc"));
        assert!(task.deleted_at.is_none());
        assert!(task.updated_at >= task.created_at);
    }

    #[test]
    fn marks_task_as_deleted_with_timestamp() {
        let mut task = Task::new("Delete me".to_string(), String::new());

        task.mark_deleted();

        assert!(task.is_deleted());
        assert_eq!(task.deleted_at, Some(task.updated_at));
    }
}
