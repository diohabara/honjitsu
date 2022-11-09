use dotenv::dotenv;
use egg_mode::tweet::DraftTweet;
use log::{error, info};
use std::env;

pub async fn create_tweet(text: &str) {
    env_logger::init();
    dotenv().ok();
    let consumer_key = env::var("TWITTER_CONSUMER_KEY").expect("TOGGL_EMAIL must be set");
    let consumer_secret = env::var("TWITTER_CONSUMER_SECRET").expect("TOGGL_PASSWORD must be set");
    let access_token = env::var("TWITTER_ACCESS_TOKEN").expect("TOGGL_EMAIL must be set");
    let access_token_secret =
        env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TOGGL_PASSWORD must be set");
    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: egg_mode::KeyPair::new(access_token, access_token_secret),
    };
    let tweet = DraftTweet::new(text.to_string());
    let res = tweet.send(&token).await;
    match res {
        Ok(tweet) => info!("tweeted: {}", tweet.text),
        Err(err) => error!("tweet failed: {}", err),
    }
}
