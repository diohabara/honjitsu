use honjitsu::{
    scrapbox::get_scrapbox_yesterday_entry, todoist::get_today_todoist_completed_tasks,
};

extern crate tokio;
mod toggl;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let pair_of_description = toggl::get_today_toggl_time_entries().await?;
    for (k, v) in pair_of_description.iter() {
        println!("{}: {}h {}m", k, v.num_hours(), v.num_minutes());
    }
    get_today_todoist_completed_tasks().await?;
    let scrapbox_text = get_scrapbox_yesterday_entry().await?;
    println!("{}", scrapbox_text);
    Ok(())
}
