# Quickstart - Web

1. Follow [native quickstart](../native/index.md)
1. install the helper cli: `cargo install forky_cli`
	- More details on the [cli page](../../forky_cli/sweet.md)
2. Update your test to do some web stuff:
	```rs
	web_sys::window().unwrap()
			.document().unwrap()
			.body().unwrap()
			.set_inner_html("<h1>This is a heading</h1>");
	expect(window).get("h1")?
		.to_contain_text("This is a heading")?;
	```
3. run `forky sweet`
	- requires [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)

Here's an example of a runner with a few tests:

![wasm-runner](../images/wasm-runner.png)


> Note: to disable the below warning its helpful to enable [this chrome feature](chrome://flags/#privacy-sandbox-ads-apis)
> 
> `Error with Permissions-Policy header: Origin trial controlled feature not enabled: 'browsing-topics'.`