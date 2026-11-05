use std::{thread::sleep, time::Duration};
use cliclack::{self, spinner, input};
use tokio;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop{

    let city: String = input("What city you want to know the temprature of: ")
        .placeholder("Not sure")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
    }).interact()?;
        let spinner = spinner();
        let client = reqwest::Client::new();
        spinner.start("Fetching");
        let url = format!("https://wttr.in/{}?format=3", city);
        let body = client.get(url).send().await?.text().await?;
        println!("weather: {}", body);
        sleep(Duration::from_secs(3));
    }
}