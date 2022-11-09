use chrono::prelude::*;
use chrono::Duration;
use dotenv::dotenv;
use log::info;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Method;
use std::collections::HashMap;
use std::env;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
struct TimeEntry {
    at: String,
    billable: bool,
    description: String,
    duration: i64,
    duronly: bool,
    id: Option<i64>,
    pid: Option<i64>,
    project_id: Option<i64>,
    server_deleted_at: Option<String>,
    start: String,
    stop: Option<String>,
    tag_ids: Option<Vec<i64>>,
    tags: Option<Vec<String>>,
    task_id: Option<i64>,
    tid: Option<i64>,
    uid: Option<i64>,
    user_id: Option<i64>,
    wid: Option<i64>,
    workspace_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WorkspaceProject {
    active: bool,
    actual_hours: Option<i64>,
    at: String,
    auto_estimates: Option<bool>,
    billable: Option<bool>,
    cid: i64,
    client_id: Option<i64>,
    color: String,
    created_at: String,
    currency: Option<String>,
    current_period: Option<CurrentPeriod>,
    estimated_hours: Option<i64>,
    fixed_fee: Option<String>,
    id: i64,
    is_private: bool,
    name: String,
    rate: Option<i64>,
    rate_last_updated: Option<String>,
    recurring: bool,
    recurring_parameters: Option<String>,
    server_deleted_at: Option<String>,
    template: Option<bool>,
    wid: i64,
    workspace_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct CurrentPeriod {
    end_date: String,
    start_date: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RecurringParameters {
    description: String,
    items: Vec<RecurringParametersItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RecurringParametersItem {
    custom_period: i64,
    estimated_seconds: i64,
    parameter_end_date: Option<String>,
    parameter_start_date: String,
    period: String,
    project_start_date: String,
}

async fn get_project_name_of_entry(
    workspace_id: i64,
    project_id: i64,
) -> Result<String, reqwest::Error> {
    info!("get_project_name_of_entry");
    let url = format!(
        "https://api.track.toggl.com/api/v9/workspaces/{workspace_id}/projects/{project_id}"
    );
    dotenv().ok();
    let email = env::var("TOGGL_EMAIL").expect("TOGGL_EMAIL must be set");
    let password = env::var("TOGGL_PASSWORD").expect("TOGGL_PASSWORD must be set");
    let client = Client::new();
    let req = client
        .request(Method::GET, url.to_string())
        .basic_auth(email, Some(password))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    let req_text = req.text().await?;
    info!("req_text: {}", &req_text);
    let project: WorkspaceProject = serde_json::from_str(req_text.as_str()).unwrap();
    info!("project: {:?}", &project);
    Ok(project.name)
}

async fn get_time_entries() -> Result<Vec<TimeEntry>, reqwest::Error> {
    info!("get_time_entries");
    dotenv().ok();
    let url = "https://api.track.toggl.com/api/v9/me/time_entries";
    let email = env::var("TOGGL_EMAIL").expect("TOGGL_EMAIL must be set");
    let password = env::var("TOGGL_PASSWORD").expect("TOGGL_PASSWORD must be set");
    let client = Client::new();
    let req = client
        .request(Method::GET, url.to_string())
        .basic_auth(email, Some(password))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    let req_text = req.text().await?;
    info!("req_text: {}", &req_text);
    let time_entries: Vec<TimeEntry> = serde_json::from_str(req_text.as_str()).unwrap();
    info!("time_entries: {:?}", &time_entries);
    Ok(time_entries)
}

pub async fn get_entry_project_to_duration(
) -> Result<Vec<((String, String), Duration)>, reqwest::Error> {
    info!("get_entry_project_to_duration");
    let time_entries: Vec<TimeEntry> = get_time_entries().await?;
    let today = Utc::today();
    let mut description_to_duration: HashMap<(String, String), Duration> = HashMap::new();
    for entry in time_entries {
        let start_time = DateTime::parse_from_rfc3339(&entry.start)
            .unwrap()
            .with_timezone(&Utc);
        if start_time.date() != today {
            continue;
        }
        let workspace_id = entry.workspace_id.unwrap_or(0);
        let project_id = entry.project_id.unwrap_or(0);
        let project_name = get_project_name_of_entry(workspace_id, project_id).await?;
        match entry.stop {
            None => continue,
            Some(stop) => {
                let stop_time = DateTime::parse_from_rfc3339(&stop)
                    .unwrap()
                    .with_timezone(&Utc);
                let duration = stop_time - start_time;
                let description = entry.description;
                let key = (project_name, description);
                if !description_to_duration.contains_key(&key) {
                    description_to_duration.insert(key, duration);
                } else {
                    let old_duration = description_to_duration.get(&key).unwrap();
                    description_to_duration.insert(key, duration + *old_duration);
                }
            }
        }
    }
    let mut pair_of_description_and_duration: Vec<_> =
        description_to_duration.into_iter().collect();
    pair_of_description_and_duration.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(pair_of_description_and_duration)
}
