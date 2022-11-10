use std::vec;

use chrono::prelude::*;
use chrono::Duration;

use chrono_tz::America::Chicago;
use honjitsu::twitter::create_tweet;
use honjitsu::{toggl::get_entry_project_to_duration};
use log::info;
use rand::seq::SliceRandom;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    let mut tweet_contents: Vec<String> = Vec::new();
    let utc = Utc::now().naive_utc();
    let today = Chicago.from_utc_datetime(&utc).date();
    let yesterday = today - Duration::days(1);
    tweet_contents.push(format!("{}", yesterday.format("%Y/%m/%d")));
    let pairs = get_entry_project_to_duration(yesterday).await?;
    let mut previous_project = "".to_string();
    let emoji_candidates = vec!["💩", "✨", "😁", "🎶", "🕺"];
    let random_emoji = emoji_candidates.choose(&mut rand::thread_rng()).unwrap();
    for ((project, task), duration) in pairs.iter() {
        if &previous_project != project {
            tweet_contents.push(format!("{random_emoji}{project}"));
            previous_project = project.to_string();
        }
        tweet_contents.push(format!(
            "{task}={:02}h:{:02}m",
            duration.num_minutes() / 60,
            duration.num_minutes() % 60
        ));
    }
    let text = tweet_contents.join("\n");
    info!("{}", text);
    create_tweet(text.as_str()).await;
    Ok(())
}
