use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub enum SyncAction {
    Push,
    Delete,
    Pull,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncErrorRecord {
    pub at: DateTime<Utc>,
    pub action: SyncAction,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncProviderMetrics {
    pub provider: String,
    #[serde(default)]
    pub last_success_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub last_error_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub last_error: Option<String>,
    #[serde(default)]
    pub recent_errors: Vec<SyncErrorRecord>,
}

impl SyncProviderMetrics {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            last_success_at: None,
            last_error_at: None,
            last_error: None,
            recent_errors: Vec::new(),
        }
    }

    pub fn record_success(&mut self, at: DateTime<Utc>) {
        self.last_success_at = Some(at);
        self.last_error_at = None;
        self.last_error = None;
    }

    pub fn record_error(
        &mut self,
        action: SyncAction,
        message: impl Into<String>,
        at: DateTime<Utc>,
    ) {
        let message = message.into();
        self.last_error_at = Some(at);
        self.last_error = Some(message.clone());
        self.recent_errors.push(SyncErrorRecord {
            at,
            action,
            message,
        });
        let overflow = self.recent_errors.len().saturating_sub(5);
        if overflow > 0 {
            self.recent_errors.drain(0..overflow);
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncJob {
    pub id: Uuid,
    pub provider: String,
    #[serde(default)]
    pub task_id: Option<Uuid>,
    pub action: SyncAction,
    #[serde(default)]
    pub attempts: u32,
    #[serde(default)]
    pub next_retry_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub last_error: Option<String>,
}

impl SyncJob {
    pub fn new(provider: impl Into<String>, task_id: Uuid, action: SyncAction) -> Self {
        Self {
            id: Uuid::new_v4(),
            provider: provider.into(),
            task_id: Some(task_id),
            action,
            attempts: 0,
            next_retry_at: None,
            last_error: None,
        }
    }

    pub fn provider_job(provider: impl Into<String>, action: SyncAction) -> Self {
        Self {
            id: Uuid::new_v4(),
            provider: provider.into(),
            task_id: None,
            action,
            attempts: 0,
            next_retry_at: None,
            last_error: None,
        }
    }

    pub fn is_due(&self, now: DateTime<Utc>) -> bool {
        self.next_retry_at.is_none_or(|retry_at| retry_at <= now)
    }

    pub fn record_failure(&mut self, error: impl Into<String>, now: DateTime<Utc>) {
        self.attempts = self.attempts.saturating_add(1);
        self.last_error = Some(error.into());
        let minutes = 2u32.saturating_pow(self.attempts.min(6)).min(60);
        self.next_retry_at = Some(now + Duration::minutes(minutes as i64));
    }
}

#[cfg(test)]
mod tests {
    use super::{SyncAction, SyncJob, SyncProviderMetrics};
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn retry_backoff_sets_future_retry() {
        let now = Utc::now();
        let mut job = SyncJob::new("notion", Uuid::new_v4(), SyncAction::Push);

        job.record_failure("boom", now);

        assert_eq!(job.attempts, 1);
        assert!(!job.is_due(now));
        assert_eq!(job.last_error.as_deref(), Some("boom"));
    }

    #[test]
    fn creates_provider_level_job_without_task_id() {
        let job = SyncJob::provider_job("notion", SyncAction::Pull);

        assert_eq!(job.task_id, None);
        assert_eq!(job.action, SyncAction::Pull);
    }

    #[test]
    fn provider_metrics_keep_recent_error_history_bounded() {
        let now = Utc::now();
        let mut metrics = SyncProviderMetrics::new("notion");

        for idx in 0..7 {
            metrics.record_error(SyncAction::Push, format!("err-{idx}"), now);
        }

        assert_eq!(metrics.recent_errors.len(), 5);
        assert_eq!(metrics.recent_errors[0].message, "err-2");
        assert_eq!(metrics.last_error.as_deref(), Some("err-6"));
    }
}
