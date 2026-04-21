
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::review::{ReviewState, ReviewConfig, ReviewSubtask};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Importance {
    Low,
    Medium,
    High,
    Urgent,
}

impl Default for Importance {
    fn default() -> Self {
        Importance::Medium
    }
}

fn default_created_at() -> DateTime<Utc> {
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
    #[serde(default)]
    pub due_date: Option<DateTime<Utc>>,
    pub review_state: ReviewState,
    #[serde(default)]
    pub review_subtasks: Vec<ReviewSubtask>,
    #[serde(default)]
    pub custom_review_str: String,
    #[serde(default)]
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
            due_date: None,
            review_state: ReviewState::default(),
            review_subtasks: Vec::new(),
            custom_review_str: String::new(),
            notion_id: None,
        }
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
        if self.review_subtasks.iter().any(|s| s.completed) { return; }
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
        if self.review_subtasks.is_empty() { return; }
        
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
