use chrono::prelude::*;
use chrono::Duration;

// use chrono_tz::America::Chicago;
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use honjitsu::todoist::get_todoist_completed_tasks;
use honjitsu::toggl::get_entry_project_to_duration;
use honjitsu::toggl::TogglClientName;
use honjitsu::toggl::TogglProjectName;
use honjitsu::toggl::TogglTaskName;

use honjitsu::twitter::create_tweet;
use log::info;

fn divide_sentence_into_tweets(sentence: &str) -> Vec<String> {
    let mut tweets = Vec::new();
    let mut current_tweet = String::new();
    for word in sentence.split('\n') {
        if current_tweet.len() + word.len() < 280 {
            current_tweet.push_str(word);
            current_tweet.push('\n');
        } else {
            tweets.push(current_tweet);
            current_tweet = String::new();
            current_tweet.push_str(word);
            current_tweet.push('\n');
        }
    }
    if !current_tweet.is_empty() {
        tweets.push(current_tweet);
    }
    tweets
}

async fn get_toggl_text_for_tweet(date: Date<Tz>) -> Result<String, reqwest::Error> {
    let toggl_pairs: Vec<((TogglClientName, TogglProjectName, TogglTaskName), Duration)> =
        get_entry_project_to_duration(date).await?;
    let mut previous_project = "".to_string();
    let mut toggl_texts = Vec::new();
    toggl_texts.push(format!("Toggl {}", date.format("%Y/%m/%d")));
    for (_, ((client, project, task), duration)) in toggl_pairs.iter().enumerate() {
        if &previous_project != project {
            toggl_texts.push(format!("⏰{client}/{project}"));
            previous_project = project.to_string();
        }
        toggl_texts.push(format!(
            "{task}={:02}:{:02}",
            duration.num_minutes() / 60,
            duration.num_minutes() % 60
        ));
    }
    Ok(toggl_texts.join("\n"))
}

async fn get_todoist_text_for_tweet(date: Date<Tz>) -> Result<String, reqwest::Error> {
    let tasks = get_todoist_completed_tasks(date).await?;
    let mut todoist_texts = Vec::new();
    todoist_texts.push(format!("Todoist {}", date.format("%Y/%m/%d")));
    for t in tasks {
        todoist_texts.push(format!("✅{}", t));
    }
    Ok(todoist_texts.join("\n"))
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    let utc = Utc::now().naive_utc();
    // let today = Chicago.from_utc_datetime(&utc).date();
    let today = Tokyo.from_utc_datetime(&utc).date();
    let yesterday = today - Duration::days(1);
    let toggl_text = get_toggl_text_for_tweet(yesterday).await?;
    for tweet in divide_sentence_into_tweets(toggl_text.as_str()) {
        info!("{tweet} {}", tweet.len());
        create_tweet(tweet.as_str()).await;
    }
    let todoist_text = get_todoist_text_for_tweet(yesterday).await?;
    for tweet in divide_sentence_into_tweets(todoist_text.as_str()) {
        info!("{tweet} with the length {}", tweet.len());
        create_tweet(tweet.as_str()).await;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn divide_sentence() {
        let original = "Toggl 2023/01/04\n⏰3UTD/meeting-for\nsyssec=01:23\n⏰2Career/preparing-by\nLeetCode=00:20\n⏰2Career/interviewing-with\nCSG=00:30\n⏰1Private/working-out-by\nwalking=00:26\n⏰1Private/developing\nninety-nine OCaml=00:03\n⏰0Garbage/watching\nnetflix=02:58\n⏰0Garbage/eating\nlunch=00:50\n⏰0Garbage/documenting\nregistration of classes=00:59\nVISA update=00:08 353
";
        let achieved = super::divide_sentence_into_tweets(original);
        assert_eq!(
            achieved,
            vec!["Toggl 2023/01/04\n⏰3UTD/meeting-for\nsyssec=01:23\n⏰2Career/preparing-by\nLeetCode=00:20\n⏰2Career/interviewing-with\nCSG=00:30\n⏰1Private/working-out-by\nwalking=00:26\n⏰1Private/developing\nninety-nine OCaml=00:03\n⏰0Garbage/watching\nnetflix=02:58\n⏰0Garbage/eating\n", "lunch=00:50\n⏰0Garbage/documenting\nregistration of classes=00:59\nVISA update=00:08 353\n\n"]
        );
    }
}
