use crate::model::Task;

pub mod notion;

#[allow(dead_code)]
pub trait SyncProvider {
    fn name(&self) -> &'static str;
    fn health_check(&self) -> Result<(), String>;
    fn push_task(&self, task: &Task) -> Result<String, String>;
    fn delete_task(&self, remote_id: &str) -> Result<(), String>;
    fn pull_tasks(&self) -> Result<Vec<Task>, String>;
}
