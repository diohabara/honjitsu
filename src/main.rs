use chrono::prelude::*;
use chrono::Duration;

use chrono_tz::America::Chicago;
use chrono_tz::Tz;
use honjitsu::todoist::get_todoist_completed_tasks;
use honjitsu::toggl::get_entry_project_to_duration;
use honjitsu::twitter::create_tweet;
use log::info;

async fn get_toggle_text_for_tweet(date: Date<Tz>) -> Result<String, reqwest::Error> {
    let mut tweet_contents: Vec<String> = Vec::new();
    tweet_contents.push(format!("{}", date.format("%Y/%m/%d")));
    let toggl_pairs = get_entry_project_to_duration(date).await?;
    let mut previous_project = "".to_string();
    for ((project, task), duration) in toggl_pairs.iter() {
        if &previous_project != project {
            tweet_contents.push(format!("*{project}"));
            previous_project = project.to_string();
        }
        tweet_contents.push(format!(
            "{task}={:02}:{:02}",
            duration.num_minutes() / 60,
            duration.num_minutes() % 60
        ));
    }
    Ok(tweet_contents.join("\n"))
}

async fn get_todoist_text_for_tweet(date: Date<Tz>) -> Result<String, reqwest::Error> {
    let mut todoist_text = Vec::new();
    todoist_text.push(format!("Tasks {}", date.format("%Y/%m/%d")));
    todoist_text.append(&mut get_todoist_completed_tasks(date).await?);
    Ok(todoist_text.join("\n"))
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    let utc = Utc::now().naive_utc();
    let today = Chicago.from_utc_datetime(&utc).date();
    let yesterday = today - Duration::days(1);
    let todoist_text_for_tweet = get_todoist_text_for_tweet(yesterday).await?;
    let toggl_text_for_tweet = get_toggle_text_for_tweet(yesterday).await?;
    info!("{todoist_text_for_tweet} {}", todoist_text_for_tweet.len());
    info!("{toggl_text_for_tweet} {}", toggl_text_for_tweet.len());
    create_tweet(todoist_text_for_tweet.as_str()).await;
    create_tweet(toggl_text_for_tweet.as_str()).await;
    Ok(())
}
