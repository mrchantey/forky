use fantoccini::ClientBuilder;
use fantoccini::Locator;
use sweet::*;
use std::process::Command;

// example from fantoccini README https://crates.io/crates/fantoccini

sweet! {
	it nonSend "works" {
		let mut chromedriver = Command::new("chromedriver")
			.args(["--silent", "--port=9515"])
			.spawn()?;
		let cap = serde_json::from_str(
			r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
		)
		.unwrap();

		let client = ClientBuilder::native()
		.capabilities(cap)
		.connect("http://localhost:9515")
		.await?;

		client.goto("https://en.wikipedia.org/wiki/Foobar").await?;
		let url = client.current_url().await?;
		expect(url.as_ref()).to_be("https://en.wikipedia.org/wiki/Foobar");

		client.close().await?;
		chromedriver.kill()?;
	}
}
