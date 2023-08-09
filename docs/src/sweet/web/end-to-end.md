# End-To-End

By default web tests run *inside* the iframe. This is great for testing components, but when we want to test a page provided by the server we need an end-to-end approach.

Test cases marked as `e2e` will run in the parent process instead of inside the iframe. The child `iframe` can be retrieved via `visit()`, at which point you can interact with the underlying document just like with unit tests.

```rs
sweet!{
	it e2e "works"{
		let page = visit("localhost:7777").await; 
		expect(page).to_contain_text("sweet as!")?;
	}
}
```