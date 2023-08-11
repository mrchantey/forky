use fantoccini::ClientBuilder;
use fantoccini::Locator;
use sweet::*;

sweet! {
	it nonSend "works" {
		let c = ClientBuilder::native().connect("http://localhost:9515")
		.await.map_err(|e| anyhow::anyhow!("failed to connect, is chromedriver running?\n{:?}",e))?;

		// first, go to the Wikipedia page for Foobar
		c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
		let url = c.current_url().await?;
		assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

		// click "Foo (disambiguation)"
		c.find(Locator::Css(".mw-disambig")).await?.click().await?;

		// click "Foo Lake"
		c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

		let url = c.current_url().await?;
		assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

		c.close().await?;
	}
}
