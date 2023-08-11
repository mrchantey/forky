# Fantoccini

Sweet can be used with fantoccini or any other webdriver client. Note the `nonSend` flag, as fantoccini futures are not `Send`.

```rs
use fantoccini::ClientBuilder;
use fantoccini::Locator;
use sweet::*;

// example from fantoccini README https://crates.io/crates/fantoccini

sweet! {
	it nonSend "works" {
		let c = ClientBuilder::native().connect("http://localhost:9515")
		.await.map_err(|e| anyhow::anyhow!("failed to connect, is chromedriver running?\n{:?}",e))?;

		c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
		let url = c.current_url().await?;
		assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

		c.find(Locator::Css(".mw-disambig")).await?.click().await?;
		c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

		let url = c.current_url().await?;
		assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

		c.close().await?;
	}
}
```