# Quickstart - Web

1. Follow [native quickstart](../native/index.md)
2. Install the cli: `cargo install sweet`
	- More details on the [cli page](../cli.md)
3. Run `sweet --example my_example`
4. Optional - update your test to do some web stuff:
	```rs
	web_sys::window().unwrap()
			.document().unwrap()
			.body().unwrap()
			.set_inner_html("<h1>This is a heading</h1>");
	expect(window()).get("h1")?
		.to_contain_text("This is a heading")?;
	```


Here's an example of a runner with a few tests:

![wasm-runner](../images/wasm-runner.png)


> Note: the below noisy warning can be stopped by enabling `chrome://flags/#privacy-sandbox-ads-apis`
> 
> `Error with Permissions-Policy header: Origin trial controlled feature not enabled: 'browsing-topics'.`