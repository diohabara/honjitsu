use chrono::prelude::*;
use chrono::Duration;
use honjitsu::{toggl::get_entry_project_to_duration, twitter::create_tweet};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut tweet_contents: Vec<String> = Vec::new();
    let yesterday = (Utc::today() - Duration::days(1)).with_timezone(&Utc);
    tweet_contents.push(format!("{yesterday} summery"));
    tweet_contents.push("⏰Toggl".to_string());
    let pairs = get_entry_project_to_duration().await?;
    for ((project, task), duration) in pairs.iter() {
        tweet_contents.push(format!(
            "{task}∈{project} {:02}h:{:02}m",
            duration.num_minutes() / 60,
            duration.num_minutes() % 60
        ));
    }
    create_tweet(tweet_contents.join("\n").as_str()).await;
    Ok(())
}
