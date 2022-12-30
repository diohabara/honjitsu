use chrono::prelude::*;
use chrono::Duration;

// use chrono_tz::America::Chicago;
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use honjitsu::todoist::get_todoist_completed_tasks;
use honjitsu::toggl::get_entry_project_to_duration;

use honjitsu::twitter::create_tweet;
use log::info;

async fn get_toggle_text_for_tweet(date: Date<Tz>) -> Result<Vec<String>, reqwest::Error> {
    let mut toggl_tweets: Vec<String> = Vec::new();
    let toggl_pairs = get_entry_project_to_duration(date).await?;
    let mut previous_project = "".to_string();
    let mut current_tasks = Vec::new();
    current_tasks.push(format!("Toggl {}", date.format("%Y/%m/%d")));
    for (i, ((client, project, task), duration)) in toggl_pairs.iter().enumerate() {
        if current_tasks.is_empty() {
            current_tasks.push(format!("Toggl {}", date.format("%Y/%m/%d")));
        }
        if &previous_project != project {
            current_tasks.push(format!("⏰{client}/{project}"));
            previous_project = project.to_string();
        }
        current_tasks.push(format!(
            "{task}={:02}:{:02}",
            duration.num_minutes() / 60,
            duration.num_minutes() % 60
        ));
        // emoji(2char) + project + \n + task + =hh:mm(6char)
        if i + 1 < toggl_pairs.len()
            && 240
                < current_tasks.len()
                    + toggl_pairs[i + 1].0 .1.len()
                    + 1
                    + toggl_pairs[i + 1].0 .0.len()
                    + 6
                    + 2
        {
            toggl_tweets.push(current_tasks.join("\n"));
            current_tasks = Vec::new();
        }
    }
    if !current_tasks.is_empty() {
        toggl_tweets.push(current_tasks.join("\n"));
    }
    Ok(toggl_tweets)
}

async fn get_todoist_text_for_tweet(date: Date<Tz>) -> Result<Vec<String>, reqwest::Error> {
    let mut todoist_tweets = Vec::new();
    let tasks = get_todoist_completed_tasks(date).await?;
    let mut current_tasks = Vec::new();
    current_tasks.push(format!("Todoist {}", date.format("%Y/%m/%d")));
    for i in 0..tasks.len() {
        if current_tasks.is_empty() {
            current_tasks.push(format!("Todoist {}", date.format("%Y/%m/%d")));
        }
        current_tasks.push(format!("✅{}", tasks[i]));
        // emoji(2 characters) + task
        if i + 1 < tasks.len() && 240 < current_tasks.len() + tasks[i + 1].len() + 2 {
            todoist_tweets.push(current_tasks.join("\n"));
            current_tasks = Vec::new();
        }
    }
    if !current_tasks.is_empty() {
        todoist_tweets.push(current_tasks.join("\n"));
    }
    Ok(todoist_tweets)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    let utc = Utc::now().naive_utc();
    // let today = Chicago.from_utc_datetime(&utc).date();
    let today = Tokyo.from_utc_datetime(&utc).date();
    let yesterday = today - Duration::days(1);
    let toggl_text_for_tweets = get_toggle_text_for_tweet(yesterday).await?;
    for tweet in toggl_text_for_tweets {
        info!("{tweet} {}", tweet.len());
        create_tweet(tweet.as_str()).await;
    }
    let todoist_text_for_tweets = get_todoist_text_for_tweet(yesterday).await?;
    for tweet in todoist_text_for_tweets {
        info!("{tweet} with the length {}", tweet.len());
        create_tweet(tweet.as_str()).await;
    }
    Ok(())
}
