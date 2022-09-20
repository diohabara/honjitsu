use honjitsu::{
    scrapbox::get_scrapbox_yesterday_entry, todoist::get_yesterday_todoist_completed_tasks,
    toggl::get_yesterday_toggl_time_entries
};

extern crate tokio;
mod toggl;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("# Toggl");
    let pair_of_description = get_yesterday_toggl_time_entries().await?;
    for (k, v) in pair_of_description.iter() {
        println!("{}: {}h {}m", k, v.num_minutes()/60, v.num_minutes());
    }

    println!("# Todoist");
    let completed_tasks = get_yesterday_todoist_completed_tasks().await?;
    for k in completed_tasks.iter() {
        println!("{}", k);
    }

    println!("# Scrapbox");
    let scrapbox_text = get_scrapbox_yesterday_entry().await?;
    println!("{}", scrapbox_text);
    Ok(())
}
