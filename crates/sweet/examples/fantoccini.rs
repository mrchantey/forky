#![feature(async_closure)]
use anyhow::Result;
use fantoccini::ClientBuilder;
use forky_core::retry_async;
use std::process::Command;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
	let mut chromedriver =
		Command::new("chromedriver").args(["--port=9515"]).spawn()?;
	let client = retry_async(
		async || {
			let cap = serde_json::from_str(
				r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
			)
			.unwrap();

			ClientBuilder::native()
				.capabilities(cap)
				.connect("http://localhost:9515")
				.await
		},
		Duration::from_secs(1),
	)
	.await?;

	client.goto("http://example.com").await?;
	let url = client.current_url().await?;
	assert!(url.as_ref().contains("example.com"));

	client.close().await?;
	chromedriver.kill()?;
	Ok(())
}
