use chrono::prelude::*;
use chrono::Duration;
use regex::Regex;
use reqwest::Client;
use reqwest::Method;

pub async fn get_scrapbox_yesterday_entry() -> Result<String, reqwest::Error> {
    let re = Regex::new(
        r"(?x)
      (?P<y>\d{4}) # year
      -
      0(?P<m>\d) # month
      -
      (?P<d>\d{2}) # day
    ",
    )
    .unwrap();
    let yesterday = (Utc::today() - Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();
    let yesterday = re.replace(&yesterday, "$y%2F$m%2F$d").to_string();
    let url = format!("https://scrapbox.io/api/pages/jampon/{}/text", yesterday);
    let client = Client::new();
    let req = client.request(Method::GET, url.to_string()).send().await?;
    let res_text = req.text().await?;
    Ok(res_text)
}
