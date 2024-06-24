use std::time::Duration;
use tokio::time::sleep;
use feed_rs::parser;
use reqwest;
use chrono::prelude::*;
use serde_json::json;
use dotenv::dotenv;
use std::env;

const RSS_URL: &str = "https://developer.apple.com/news/releases/rss/releases.rss";
const CHECK_INTERVAL: Duration = Duration::from_secs(3600); // Check every hour
const TEST_MODE: bool = false; // Set this to true for testing

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file
    let discord_webhook_url = env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set in .env file");

    let client = reqwest::Client::new();
    let mut last_check_time = if TEST_MODE {
        Utc::now() - chrono::Duration::hours(24)
    } else {
        Utc::now()
    };

    loop {
        println!("Checking for updates...");
        let response = client.get(RSS_URL).send().await?;
        let feed = parser::parse(response.bytes().await?.as_ref())?;

        for entry in feed.entries {
            if entry.published.map_or(false, |date| date > last_check_time) {
                let title = entry.title.map_or("No title".to_string(), |t| t.content);
                let link = entry.links.first().map_or("No link".to_string(), |l| l.href.clone());
                let message = format!("New iOS beta update: {}\n{}", title, link);

                println!("New entry found: {}", message);

                let payload = json!({
                    "content": null,
                    "embeds": [{
                        "title": "New iOS Beta Update",
                        "description": message,
                        "color": 2067276 // Dark green color
                    }],
                    "username": "iOS Beta Update Notifier"
                });

                let response = client.post(&discord_webhook_url)
                    .json(&payload)
                    .send()
                    .await?;

                println!("Discord webhook response status: {}", response.status());
            }
        }

        if !TEST_MODE {
            last_check_time = Utc::now();
        }
        println!("Waiting for next check...");
        sleep(CHECK_INTERVAL).await;
    }
}
