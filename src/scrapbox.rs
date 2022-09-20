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

fn convert_scrapbox_icon_into_image(icon: &str) -> String {
    let re = Regex::new(r"\[([[:alnum:]]*?)\.icon\]").unwrap();
    let matched_part = re.captures(icon).unwrap();
    let url = format!(
        "![{}](https://scrapbox.io/api/pages/jampon/{}/icon)",
        &matched_part[1], &matched_part[1]
    );
    url
}

fn convert_scrapbox_link_into_url(link: &str) -> String {
    let re = Regex::new(r"\[([[:alnum:]]*?)\]").unwrap();
    let matched_part = re.captures(link).unwrap();
    let url = format!(
        "[{}](https://scrapbox.io/jampon/{})",
        &matched_part[1], &matched_part[1]
    );
    url
}

fn convert_scrapbox_asterisk_into_header(text: &str) -> String {
    let re = Regex::new(
        r"(?x)
      \[
      (?P<asterisk>\*+) # asterisk
      \s
      (?P<text>.*) # text
      \]",
    )
    .unwrap();
    let matched_part = re.captures(text).unwrap();
    let header_size = (0..(5 - matched_part[1].len()))
        .map(|_| "#")
        .collect::<String>();
    let header = format!("{} {}", &header_size, &matched_part[2]);
    header
}

fn convert_scrapbox_text_into_markdown(text: &str) -> String {
    let re = Regex::new(
        r"(?x)
      (?P<url>https?://[^\s]+) # url
      (?P<text>\[+\]) # text
    ",
    )
    .unwrap();
    let text = re.replace(text, "$text($url)").to_string();
    text
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::scrapbox::convert_scrapbox_asterisk_into_header;
    use crate::scrapbox::convert_scrapbox_date_to_url_date;
    use crate::scrapbox::convert_scrapbox_icon_into_image;
    use crate::scrapbox::convert_scrapbox_link_into_url;
    use crate::scrapbox::convert_scrapbox_text_into_markdown;

    #[test]
    fn test_convert_scrapbox_date_into_url_date() {
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

    #[test]
    fn test_convert_scrapbox_icon_into_image() {
        let pass_icon = "[pass.icon]";
        let pass_url = convert_scrapbox_icon_into_image(pass_icon);
        assert_eq!(
            pass_url,
            "![pass](https://scrapbox.io/api/pages/jampon/pass/icon)"
        );
    }

    #[test]
    fn test_convert_scrapbox_link_into_url() {
        let pass_link = "[pass]";
        let pass_url = convert_scrapbox_link_into_url(pass_link);
        assert_eq!(pass_url, "[pass](https://scrapbox.io/jampon/pass)");
    }

    #[test]
    fn test_convert_scrapbox_asterisk_into_header() {
        let asterisk = "[** text]";
        let header = convert_scrapbox_asterisk_into_header(asterisk);
        assert_eq!(header, "### text");
        let asterisk = "[*** text]";
        let header = convert_scrapbox_asterisk_into_header(asterisk);
        assert_eq!(header, "## text");
    }

    #[test]
    #[ignore]
    fn test_convert_scrapbox_text_into_markdown() {
        let given = r#"
        2022/9/15
#report
[*** TODO]
 [v] LeetCode 5
 [v] Recap [CS 5333 Discrete Structures]
 [v] [Citadel] OA
 [v] [Valkyrie Trading] OA
[*** Logs]
        LeetCode
                [fail.icon] https://leetcode.com/problems/kth-largest-element-in-an-array/
                [fail.icon] https://leetcode.com/problems/task-scheduler/
                [fail.icon] https://leetcode.com/problems/design-twitter/
                [pass.icon] https://leetcode.com/problems/subsets/
                [fail.icon] https://leetcode.com/problems/combination-sum/
        Recap [CS 5333 Discrete Structures]
        [Citadel] OA
                [jio.icon] done
                [jio.icon] 2/2 solved
        [Valkyrie Trading] OA
                [jio.icon] 2/2 solved
        Recap [CS 5333 Discrete Structures]
                [2022/9/12]
                [2022/9/14]
                [jio.icon] I know all about discrete math
        Improve [honjitsu]
        "#;
        let expected = r#"
        2022/9/15
#report
## TODO
 [v] LeetCode 5
 [v] Recap [CS 5333 Discrete Structures]
 [v] [Citadel] OA
 [v] [Valkyrie Trading] OA
## Logs
        LeetCode
                ![fail](https://scrapbox.io/api/pages/jampon/fail/icon) https://leetcode.com/problems/kth-largest-element-in-an-array/
        Recap [CS 5333 Discrete Structures]
        [Valkyrie Trading] OA
                [jio.icon] 2/2 solved
        Recap [CS 5333 Discrete Structures]
                [2022/9/12]
                [2022/9/14]
                [jio.icon] I know all about discrete math
        Improve [honjitsu]
        "#;
        let achieved = crate::scrapbox::convert_scrapbox_text_into_markdown(given);
        assert_eq!(expected, achieved);
    }
}
