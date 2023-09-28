# Web Driver

> This section is for native end-to-end tests. For in-browser end-to-end tests see [end-to-end](../web/end-to-end.md).


Sweet can be used with fantoccini or any other webdriver client. 

## Example

Note the `non_send` flag, as fantoccini futures are not `Send`.

```rs
use fantoccini::ClientBuilder;
use fantoccini::Locator;
use sweet::*;

sweet! {
	it non_send "works" {
		let client = ClientBuilder::native()
			.connect("http://localhost:9515").await?;

		client.goto("https://example.com").await?;
		let url = client.current_url().await?;
		expect(url.as_ref()).to_be("https://example.com")?;

		client.close().await?;
	}
}
```