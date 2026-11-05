use std::{thread::sleep, time::Duration};

use tokio;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop{
        let client = reqwest::Client::new();
        let body = client.get("https://wttr.in/Casablanca?format=3").send().await?.text().await?;
        println!("weather: {}", body);
        sleep(Duration::from_secs(3));
    }
}