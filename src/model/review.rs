use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewConfig {
    pub intervals: Vec<u32>, // Days: [1, 3, 7, 14, 30]
}

impl Default for ReviewConfig {
    fn default() -> Self {
        Self {
            intervals: vec![1, 3, 7, 14, 30],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewState {
    pub current_step: usize,
    pub last_review: Option<DateTime<Utc>>,
    pub next_review: Option<DateTime<Utc>>,
    pub completed_count: usize,
}

impl Default for ReviewState {
    fn default() -> Self {
        Self {
            current_step: 0,
            last_review: None,
            next_review: None,
            completed_count: 0,
        }
    }
}

impl ReviewState {
    pub fn advance(&mut self, config: &ReviewConfig) {
        let now = Utc::now();
        self.last_review = Some(now);
        self.completed_count += 1;

        if self.current_step < config.intervals.len() {
            let days = config.intervals[self.current_step] as i64;
            self.next_review = Some(now + Duration::days(days));
            self.current_step += 1;
        } else {
            // If finished all steps, maybe set a long defaults or cap it
            let last_interval = *config.intervals.last().unwrap_or(&30) as i64;
            self.next_review = Some(now + Duration::days(last_interval));
        }
    }

    pub fn is_due(&self) -> bool {
        match self.next_review {
            Some(next) => Utc::now() >= next,
            None => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSubtask {
    pub id: uuid::Uuid,
    pub label: String,
    pub date: DateTime<Utc>,
    pub completed: bool,
}
