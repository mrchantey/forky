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

		let c = ClientBuilder::native()
		.capabilities(cap)
		.connect("http://localhost:9515")
		.await.map_err(|e| anyhow::anyhow!("failed to connect, is chromedriver running?\n{:?}",e))?;

		c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
		let url = c.current_url().await?;
		assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

		c.find(Locator::Css(".mw-disambig")).await?.click().await?;
		c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

		let url = c.current_url().await?;
		assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

		c.close().await?;
		chromedriver.kill()?;
	}
}
