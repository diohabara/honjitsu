use chrono::prelude::*;
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
struct TimeEntry {
    at: String,
    billable: bool,
    description: String,
    duration: i64,
    duronly: bool,
    id: Option<usize>,
    pid: Option<usize>,
    project_id: Option<usize>,
    server_deleted_at: Option<String>,
    start: String,
    stop: Option<String>,
    tag_ids: Option<Vec<usize>>,
    tags: Option<Vec<String>>,
    task_id: Option<usize>,
    tid: Option<usize>,
    uid: Option<usize>,
    user_id: Option<usize>,
    wid: Option<usize>,
    workspace_id: Option<usize>,
}

pub async fn get_yesterday_toggl_time_entries() -> Result<Vec<(String, Duration)>, reqwest::Error> {
    let url = "https://api.track.toggl.com/api/v9/me/time_entries";
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
    let res_text = req.text().await?;
    let json: Vec<TimeEntry> = serde_json::from_str(res_text.as_str()).unwrap();

    let today = Utc::today();
    let mut description_to_duration: HashMap<String, Duration> = HashMap::new();
    for obj in json {
        let start_time = DateTime::parse_from_rfc3339(&obj.start)
            .unwrap()
            .with_timezone(&Utc);
        if start_time.date() < today {
            continue;
        }
        match obj.stop {
            None => continue,
            Some(stop) => {
                let stop_time = DateTime::parse_from_rfc3339(&stop)
                    .unwrap()
                    .with_timezone(&Utc);
                let duration = stop_time - start_time;
                let description = obj.description;
                if !description_to_duration.contains_key(&description.to_string()) {
                    description_to_duration.insert(description.to_string(), duration);
                } else {
                    let old_duration = description_to_duration
                        .get(&description.to_string())
                        .unwrap();
                    description_to_duration
                        .insert(description.to_string(), duration + *old_duration);
                }
            }
        }
    }
    let mut pair_of_description_and_duration: Vec<_> =
        description_to_duration.into_iter().collect();
    pair_of_description_and_duration.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(pair_of_description_and_duration)
}
