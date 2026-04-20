use crate::model::{Task, settings::AppSettings};
use serde_json::json;

#[allow(dead_code)]
pub fn sync_task_to_notion(settings: &AppSettings, task: &Task) -> Result<String, String> {
    let api_key = settings.notion_api_key.as_deref().ok_or("No API Key")?;
    let database_id = settings.notion_database_id.as_deref().ok_or("No Database ID")?;

    let url = if let Some(id) = &task.notion_id {
        format!("https://api.notion.com/v1/pages/{}", id)
    } else {
        "https://api.notion.com/v1/pages".to_string()
    };

    let payload = if task.notion_id.is_some() {
        // Update
        json!({
            "properties": {
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
            }
        })
    } else {
        // Create
        json!({
            "parent": { "database_id": database_id },
            "properties": {
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
            }
        })
    };

    let agent = ureq::Agent::new_with_defaults();
    
    let request = if task.notion_id.is_some() {
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
        let body: serde_json::Value = resp.body_mut().read_json().map_err(|e: ureq::Error| e.to_string())?;
        let notion_id = body["id"].as_str().ok_or("No ID in response")?.to_string();
        Ok(notion_id)
    } else {
        Err(format!("Error status: {}", resp.status()))
    }
}
