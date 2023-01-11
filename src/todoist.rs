use chrono::prelude::*;
// use chrono_tz::America::Chicago;
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use dotenv::dotenv;
use log::debug;
use log::error;
use regex::Regex;
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
    user_id: String,
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

#[derive(Debug, Deserialize, Serialize, Default)]
struct Value {
    items: Vec<CompletedTask>,
    projects: HashMap<String, Projects>,
}

// ref: https://developer.todoist.com/sync/v9/#get-all-completed-items
// get all completed tasks
// curl https://api.todoist.com/sync/v9/completed/get_all -H "Authorization: Bearer $token"
pub async fn get_todoist_completed_tasks(date: Date<Tz>) -> Result<Vec<String>, reqwest::Error> {
    let url = "https://api.todoist.com/sync/v9/completed/get_all";
    dotenv().ok();
    let token = env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN must be set");
    let client = Client::new();
    let response = client
        .request(Method::GET, url.to_string())
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    debug!("Response: {:?}", response);
    let response_text = response.text().await?;
    debug!("Response text: {:?}", response_text);
    let json = match serde_json::from_str::<Value>(response_text.as_str()) {
        Ok(json) => json,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(vec![]);
        }
    };
    debug!("json: {:?}", json);
    let Value { items, projects: _ } = json;
    let tasks = items;
    let re = Regex::new(r"(?x) # remove link
        \[(?P<title>.*)\] # [title]
        (?P<link>\(.*\)) # (link)
    ").unwrap();
    let mut completed_tasks = Vec::new();
    for obj in tasks {
        let completed_at = DateTime::parse_from_rfc3339(&obj.completed_at)
            .unwrap()
            .with_timezone(&Tokyo);
        if completed_at.date() == date {
            completed_tasks.push(re.replace_all(&obj.content, "$title").to_string());
        }
    }
    Ok(completed_tasks)
}
