use crate::integrations::SyncProvider;
use crate::model::{settings::AppSettings, task::Importance, Task};
use chrono::{DateTime, Utc};
use serde_json::json;

pub fn sync_task_to_notion(settings: &AppSettings, task: &Task) -> Result<String, String> {
    let api_key = settings.notion_api_key.as_deref().ok_or("No API Key")?;
    let database_id = settings
        .notion_database_id
        .as_deref()
        .ok_or("No Database ID")?;

    let url = if let Some(id) = task.remote_id("notion") {
        format!("https://api.notion.com/v1/pages/{}", id)
    } else {
        "https://api.notion.com/v1/pages".to_string()
    };

    let payload = notion_payload(task, task.remote_id("notion").is_none(), database_id);

    let agent = ureq::Agent::new_with_defaults();

    let request = if task.remote_id("notion").is_some() {
        agent.patch(&url)
    } else {
        agent.post(&url)
    };

    let mut resp = request
        .header("Authorization", &format!("Bearer {}", api_key))
        .header("Notion-Version", "2022-06-28")
        .header("Content-Type", "application/json")
        .send_json(payload)
        .map_err(|e: ureq::Error| e.to_string())?;

    if resp.status().is_success() {
        let body: serde_json::Value = resp
            .body_mut()
            .read_json()
            .map_err(|e: ureq::Error| e.to_string())?;
        let notion_id = body["id"].as_str().ok_or("No ID in response")?.to_string();
        Ok(notion_id)
    } else {
        Err(format!("Error status: {}", resp.status()))
    }
}

pub fn archive_task_in_notion(settings: &AppSettings, remote_id: &str) -> Result<(), String> {
    let api_key = settings.notion_api_key.as_deref().ok_or("No API Key")?;
    let url = format!("https://api.notion.com/v1/pages/{}", remote_id);
    let agent = ureq::Agent::new_with_defaults();
    let resp = agent
        .patch(&url)
        .header("Authorization", &format!("Bearer {}", api_key))
        .header("Notion-Version", "2022-06-28")
        .header("Content-Type", "application/json")
        .send_json(json!({ "archived": true }))
        .map_err(|e: ureq::Error| e.to_string())?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error status: {}", resp.status()))
    }
}

pub fn pull_tasks_from_notion(settings: &AppSettings) -> Result<Vec<Task>, String> {
    let api_key = settings.notion_api_key.as_deref().ok_or("No API Key")?;
    let database_id = settings
        .notion_database_id
        .as_deref()
        .ok_or("No Database ID")?;
    let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);
    let agent = ureq::Agent::new_with_defaults();
    let mut resp = agent
        .post(&url)
        .header("Authorization", &format!("Bearer {}", api_key))
        .header("Notion-Version", "2022-06-28")
        .header("Content-Type", "application/json")
        .send_json(json!({}))
        .map_err(|e: ureq::Error| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Error status: {}", resp.status()));
    }

    let body: serde_json::Value = resp
        .body_mut()
        .read_json()
        .map_err(|e: ureq::Error| e.to_string())?;
    parse_notion_tasks(&body)
}

fn notion_payload(task: &Task, include_parent: bool, database_id: &str) -> serde_json::Value {
    let properties = json!({
        "Name": {
            "title": [{ "text": { "content": task.title } }]
        },
        "Status": {
            "checkbox": task.completed
        },
        "Importance": {
            "select": { "name": format!("{:?}", task.importance) }
        },
        "Description": {
            "rich_text": [{ "text": { "content": task.description } }]
        }
    });

    if include_parent {
        json!({
            "parent": { "database_id": database_id },
            "properties": properties
        })
    } else {
        json!({ "properties": properties })
    }
}

fn parse_notion_tasks(body: &serde_json::Value) -> Result<Vec<Task>, String> {
    let results = body["results"]
        .as_array()
        .ok_or("Notion response did not contain results")?;
    let mut tasks = Vec::new();

    for page in results {
        if page["archived"].as_bool().unwrap_or(false) {
            continue;
        }
        let properties = &page["properties"];
        let title = properties["Name"]["title"]
            .as_array()
            .and_then(|items| items.first())
            .and_then(|item| {
                item["plain_text"]
                    .as_str()
                    .or_else(|| item["text"]["content"].as_str())
            })
            .unwrap_or("Untitled")
            .to_string();
        let description = properties["Description"]["rich_text"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        item["plain_text"]
                            .as_str()
                            .or_else(|| item["text"]["content"].as_str())
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .unwrap_or_default();
        let mut task = Task::new(title, description);
        task.completed = properties["Status"]["checkbox"].as_bool().unwrap_or(false);
        task.importance = match properties["Importance"]["select"]["name"]
            .as_str()
            .unwrap_or("Medium")
        {
            "Urgent" => Importance::Urgent,
            "High" => Importance::High,
            "Low" => Importance::Low,
            _ => Importance::Medium,
        };
        if let Some(id) = page["id"].as_str() {
            task.set_remote_id("notion", id.to_string());
        }
        if let Some(created_at) = page["created_time"]
            .as_str()
            .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
        {
            task.created_at = created_at.with_timezone(&Utc);
        }
        if let Some(updated_at) = page["last_edited_time"]
            .as_str()
            .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
        {
            task.updated_at = updated_at.with_timezone(&Utc);
        }
        tasks.push(task);
    }

    Ok(tasks)
}

pub struct NotionProvider {
    settings: AppSettings,
}

impl NotionProvider {
    pub fn new(settings: AppSettings) -> Self {
        Self { settings }
    }
}

impl SyncProvider for NotionProvider {
    fn name(&self) -> &'static str {
        "notion"
    }

    fn health_check(&self) -> Result<(), String> {
        self.settings
            .notion_api_key
            .as_deref()
            .ok_or("No API Key")?;
        self.settings
            .notion_database_id
            .as_deref()
            .ok_or("No Database ID")?;
        Ok(())
    }

    fn push_task(&self, task: &Task) -> Result<String, String> {
        sync_task_to_notion(&self.settings, task)
    }

    fn delete_task(&self, _remote_id: &str) -> Result<(), String> {
        archive_task_in_notion(&self.settings, _remote_id)
    }

    fn pull_tasks(&self) -> Result<Vec<Task>, String> {
        pull_tasks_from_notion(&self.settings)
    }
}

#[cfg(test)]
mod tests {
    use super::{notion_payload, parse_notion_tasks};
    use crate::model::Task;

    #[test]
    fn create_payload_includes_parent_database() {
        let task = Task::new("Ship it".to_string(), "With tests".to_string());

        let payload = notion_payload(&task, true, "db-1");

        assert_eq!(payload["parent"]["database_id"], "db-1");
        assert_eq!(
            payload["properties"]["Name"]["title"][0]["text"]["content"],
            "Ship it"
        );
    }

    #[test]
    fn update_payload_omits_parent_database() {
        let task = Task::new("Ship it".to_string(), String::new());

        let payload = notion_payload(&task, false, "db-1");

        assert!(payload.get("parent").is_none());
        assert!(payload.get("properties").is_some());
    }

    #[test]
    fn parses_notion_query_results_into_tasks() {
        let body = serde_json::json!({
            "results": [
                {
                    "id": "page-1",
                    "archived": false,
                    "created_time": "2026-04-20T10:00:00.000Z",
                    "last_edited_time": "2026-04-21T11:30:00.000Z",
                    "properties": {
                        "Name": {
                            "title": [{ "plain_text": "Task from Notion" }]
                        },
                        "Description": {
                            "rich_text": [{ "plain_text": "Details" }]
                        },
                        "Status": { "checkbox": true },
                        "Importance": { "select": { "name": "High" } }
                    }
                },
                {
                    "id": "archived",
                    "archived": true,
                    "properties": {}
                }
            ]
        });

        let tasks = parse_notion_tasks(&body).unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Task from Notion");
        assert_eq!(tasks[0].description, "Details");
        assert_eq!(tasks[0].remote_id("notion"), Some("page-1"));
        assert!(tasks[0].completed);
        assert_eq!(
            tasks[0].created_at.to_rfc3339(),
            "2026-04-20T10:00:00+00:00"
        );
        assert_eq!(
            tasks[0].updated_at.to_rfc3339(),
            "2026-04-21T11:30:00+00:00"
        );
    }
}
