# Async Tests

Internally each native test will be stored as one of three types:

- `fn() -> Result<()>`
	```rs
	it "has no 'async' keywords"{
		expect(true).to_be_true()?;
	}
	```
- `fn() -> BoxedFutureSend`
	```rs
	it "has send futures"{
		visit("http://example.com").await?;
	}
	```
- `fn() -> BoxedFuture`
	```rs
	it nonSend "has non-send futures"{
		//example of a common non-send async function
		fantoccini::ClientBuilder::native().connect("http://example.com").await
	}
	```

By default Sweet will expect any tests containing the `await` keyword to be `Send`, but it currently can't determine if the future is not `Send`. For this reason the `nonSend` flag must be specified if your test contains a non-send future.

Note: currently if the runner finds any `nonSend` tests it will run all tests on the main thread, even if the parallel flag is supplied.