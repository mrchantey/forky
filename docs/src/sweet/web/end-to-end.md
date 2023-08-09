# End-To-End

By default web tests run *inside* the iframe. This is great for testing components, but when we want to test a page provided by the server we need a different approach.

Test cases marked as `e2e` will run in the parent process instead of inside the iframe. The child `iframe` can be retrieved via `visit()`, at which point you can interact with the underlying document just like with unit tests.

Testing iframes from different origins, ie another port on localhost, is [hard](https://docs.cypress.io/guides/guides/web-security#:~:text=To%20get%20around%20these%20restrictions,rules%20of%20same%2Dorigin%20policy.). To make this easier, sweet provides a reverse proxy that will serve your iframe content from the same origin. Its been tested on simple sites like these docs, but if you encounter any problems please create an issue.

### `visit()`

visit does three things:
1. Points the proxy to the provided url
2. Sets the iframe `src` to the proxy url
   - fyi this is will be something like `/_proxy_/http://localhost:3000`
3. awaits the iframe `load` event

### Example

Here's an example of an end-to-end test running on these docs:
```rs
sweet!{
	test e2e "docs origin" {
		let page = visit("http://localhost:3000").await?;
		expect(page)
			.poll(|p|p.to_contain_text("Forky")).await?;
	}
}
```

And the output looks like this:

![end-to-end](../images/end-to-end.png)

