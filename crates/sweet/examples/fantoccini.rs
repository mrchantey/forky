use fantoccini::ClientBuilder;
use anyhow::Result;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
	let mut chromedriver =
		Command::new("chromedriver").args(["--port=9515"]).spawn()?;
	std::thread::sleep(std::time::Duration::from_secs(1));
	let client = ClientBuilder::native()
		.connect("http://localhost:9515")
		.await?;

	client.goto("http://example.com").await?;
	let url = client.current_url().await?;
	assert_eq!(url.as_ref(),"http://example.com/");

	client.close().await?;
	chromedriver.kill()?;
	Ok(())
}
