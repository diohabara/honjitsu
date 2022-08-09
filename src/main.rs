use dotenv::dotenv;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
    let toggl_api_token = std::env::var("TOGGL_API_TOKEN").expect("TOGGL_API_TOKEN must be set.");
    println!("{}", toggl_api_token);
}
