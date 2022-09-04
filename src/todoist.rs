extern crate dotenv;
extern crate serde_json;
extern crate tokio;

use chrono::prelude::*;
#[warn(unused_imports)]
use chrono::Duration;
use dotenv::dotenv;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Method;
use std::collections::HashMap;
use std::env;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: usize,
    content: String,
    completed: bool,
    label_ids: Vec<usize>,
    order: usize,
    priority: usize,
    project_id: usize,
    section_id: usize,
    url: String,
    comment_count: usize,
}

// ref: https://developer.todoist.com/sync/v9/#get-all-completed-items
// get all completed tasks
// curl https://api.todoist.com/sync/v9/completed/get_all -H "Authorization: Bearer $token"
pub async fn get_today_todoist_completed_tasks() -> Result<Vec<String>, reqwest::Error> {
    let url = "https://api.todoist.com/sync/v9/completed/get_all";
    dotenv().ok();
    let token = env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN must be set");
    let client = Client::new();
    let req = client
        .request(Method::GET, url.to_string())
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    unimplemented!("todoist");
}
