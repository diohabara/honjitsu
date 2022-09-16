use chrono::prelude::*;
use chrono::Duration;
use dotenv::dotenv;
use reqwest::Client;
use reqwest::Method;
use std::collections::HashMap;
use std::env;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
struct CompletedTask {
    completed_at: String,
    content: String,
    id: String,
    meta_data: Option<String>,
    project_id: String,
    task_id: String,
    user_id: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Projects {
    child_order: usize,
    collapsed: bool,
    color: String,
    id: String,
    inbox_project: bool,
    is_archived: bool,
    is_deleted: bool,
    is_favorite: bool,
    name: String,
    parent_id: Option<String>,
    shared: bool,
    sync_id: Option<String>,
    view_style: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Value {
    items: Vec<CompletedTask>,
    projects: HashMap<String, Projects>,
}

// ref: https://developer.todoist.com/sync/v9/#get-all-completed-items
// get all completed tasks
// curl https://api.todoist.com/sync/v9/completed/get_all -H "Authorization: Bearer $token"
pub async fn get_yesterday_todoist_completed_tasks() -> Result<Vec<String>, reqwest::Error> {
    let params = [("since", "2022-9-05T00:00:00")];
    let url = "https://api.todoist.com/sync/v9/completed/get_all";
    let url = reqwest::Url::parse_with_params(url, &params).unwrap();
    dotenv().ok();
    let token = env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN must be set");
    let client = Client::new();
    let req = client
        .request(Method::GET, url.to_string())
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let res_text = req.text().await?;
    let json: Value = serde_json::from_str(res_text.as_str()).unwrap();
    let yesterday = Utc::today() - Duration::days(1);
    let Value { items, projects: _ } = json;
    let tasks = items;
    let mut completed_tasks = Vec::new();
    for obj in tasks {
        let completed_at = DateTime::parse_from_rfc3339(&obj.completed_at)
            .unwrap()
            .with_timezone(&Utc);
        if completed_at.date() == yesterday {
            completed_tasks.push(obj.content);
        }
    }
    Ok(completed_tasks)
}
