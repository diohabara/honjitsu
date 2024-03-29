use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::America::Chicago;
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use dotenv::dotenv;

use log::debug;
use log::error;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Method;
use std::collections::HashMap;
use std::env;

use serde::Deserialize;
use serde::Serialize;

pub type TogglClientName = String;
pub type TogglProjectName = String;
pub type TogglTaskName = String;

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

#[derive(Debug, Deserialize, Serialize, Default)]
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
    recurring_parameters: Option<String>, // TODO: RecurringParameters
    server_deleted_at: Option<String>,
    template: Option<bool>,
    wid: i64,
    workspace_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
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

#[derive(Debug, Deserialize, Serialize)]
struct TogglClient {
    archived: bool,
    at: String,
    id: i64,
    name: String,
    server_deleted_at: Option<String>,
    wid: i64,
}

async fn get_project_of_entry(
    workspace_id: i64,
    project_id: i64,
) -> Result<WorkspaceProject, reqwest::Error> {
    let url = format!(
        "https://api.track.toggl.com/api/v9/workspaces/{workspace_id}/projects/{project_id}"
    );
    dotenv().ok();
    let email = env::var("TOGGL_EMAIL").expect("TOGGL_EMAIL must be set");
    let password = env::var("TOGGL_PASSWORD").expect("TOGGL_PASSWORD must be set");
    let client = Client::new();
    let response = client
        .request(Method::GET, url.to_string())
        .basic_auth(email, Some(password))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    debug!("Response: {:?}", response);
    let response_text = response.text().await?;
    let project: WorkspaceProject = match serde_json::from_str(response_text.as_str()) {
        Ok(project) => project,
        Err(e) => {
            error!("WorkspaceProject: {:?}", e);
            error!("Response: {:?}", response_text);
            WorkspaceProject::default()
        }
    };
    Ok(project)
}

async fn get_client_name_of_workspace(
    workspace_id: i64,
    client_id: i64,
) -> Result<TogglClientName, reqwest::Error> {
    let url =
        format!("https://api.track.toggl.com/api/v9/workspaces/{workspace_id}/clients/{client_id}");
    dotenv().ok();
    let email = env::var("TOGGL_EMAIL").expect("TOGGL_EMAIL must be set");
    let password = env::var("TOGGL_PASSWORD").expect("TOGGL_PASSWORD must be set");
    let client = Client::new();
    let response = client
        .request(Method::GET, url.to_string())
        .basic_auth(email, Some(password))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    debug!("Response: {:?}", response);
    let response_text = response.text().await?;
    let toggl_client: TogglClient = match serde_json::from_str(response_text.as_str()) {
        Ok(toggl_client) => toggl_client,
        Err(e) => {
            error!("TogglClient: {:?}", e);
            return Ok("".to_string());
        }
    };
    Ok(toggl_client.name)
}

async fn get_time_entries() -> Result<Vec<TimeEntry>, reqwest::Error> {
    dotenv().ok();
    let url = "https://api.track.toggl.com/api/v9/me/time_entries";
    let email = env::var("TOGGL_EMAIL").expect("TOGGL_EMAIL must be set");
    let password = env::var("TOGGL_PASSWORD").expect("TOGGL_PASSWORD must be set");
    let client = Client::new();
    let response = client
        .request(Method::GET, url.to_string())
        .basic_auth(email, Some(password))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    debug!("response: {:?}", response);
    let response_text = response.text().await?;
    let time_entries: Vec<TimeEntry> = match serde_json::from_str(response_text.as_str()) {
        Ok(times_entries) => times_entries,
        Err(e) => {
            error!("TimeEntry: {:?}", e);
            Vec::new()
        }
    };
    Ok(time_entries)
}

pub async fn get_entry_project_to_duration(
    date: Date<Tz>,
) -> Result<Vec<((TogglClientName, TogglProjectName, TogglTaskName), Duration)>, reqwest::Error> {
    let time_entries: Vec<TimeEntry> = get_time_entries().await?;
    let mut project_and_task_to_duration: HashMap<
        (TogglClientName, TogglProjectName, TogglTaskName),
        Duration,
    > = HashMap::new();
    for entry in time_entries {
        let start_time = DateTime::parse_from_rfc3339(&entry.start)
            .unwrap()
            // FIXME: change according to your timezone
            // .with_timezone(&Tokyo);
            .with_timezone(&Chicago);
        if start_time.date() != date {
            continue;
        }
        let workspace_id = entry.workspace_id.unwrap_or(0);
        let project_id = entry.project_id.unwrap_or(0);
        let project = get_project_of_entry(workspace_id, project_id).await?;
        let client_id = project.client_id.unwrap_or(0);
        let project_name = project.name;
        let client_name = get_client_name_of_workspace(workspace_id, client_id).await?;
        match entry.stop {
            None => continue,
            Some(stop) => {
                let stop_time = DateTime::parse_from_rfc3339(&stop)
                    .unwrap()
                    // FIXME: change according to your timezone
                    // .with_timezone(&Tokyo);
                    .with_timezone(&Chicago);
                let duration = stop_time - start_time;
                let description = entry.description;
                let key = (client_name, project_name, description);
                if !project_and_task_to_duration.contains_key(&key) {
                    project_and_task_to_duration.insert(key, duration);
                } else {
                    let old_duration = project_and_task_to_duration.get(&key).unwrap();
                    project_and_task_to_duration.insert(key, duration + *old_duration);
                }
            }
        }
    }
    let mut pair_of_descriptions_to_duration: Vec<_> =
        project_and_task_to_duration.into_iter().collect();
    pair_of_descriptions_to_duration.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(pair_of_descriptions_to_duration)
}
