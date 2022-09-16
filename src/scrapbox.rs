use chrono::prelude::*;
use chrono::Duration;
use regex::Regex;
use reqwest::Client;
use reqwest::Method;

pub async fn get_scrapbox_yesterday_entry() -> Result<String, reqwest::Error> {
    let yesterday = (Utc::today() - Duration::days(1))
        .format("%Y-%-m-%-d")
        .to_string();
    let yesterday = convert_scrapbox_date_to_url_date(&yesterday).to_string();
    let url = format!("https://scrapbox.io/api/pages/jampon/{}/text", yesterday);
    let client = Client::new();
    let req = client.request(Method::GET, url.to_string()).send().await?;
    let res_text = req.text().await?;
    Ok(res_text)
}

fn convert_scrapbox_date_to_url_date(date: &str) -> String {
    let re = Regex::new(
        r"(?x)
      (?P<y>\d{4}) # year
      -
      (?P<m>\d{1,2}) # month
      -
      (?P<d>\d{1,2}) # day
    ",
    )
    .unwrap();
    let date = re.replace(date, "$y%2F$m%2F$d").to_string();
    date
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::scrapbox::convert_scrapbox_date_to_url_date;

    #[test]
    fn test_convert_scrapbox_date_to_url_date() {
        let date_leading_zero = "2020-1-1";
        let url_date_leading_zero = convert_scrapbox_date_to_url_date(date_leading_zero);
        assert_eq!(url_date_leading_zero, "2020%2F1%2F1");

        let date_non_leading_zero = "2020-11-11";
        let url_date_non_leading_zero = convert_scrapbox_date_to_url_date(date_non_leading_zero);
        assert_eq!(url_date_non_leading_zero, "2020%2F11%2F11");

        let today = Utc::today().format("%Y-%-m-%-d").to_string();
        let url_today = convert_scrapbox_date_to_url_date(&today);
        assert_eq!(url_today, today.replace('-', "%2F"));
    }
}
