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
		tokio::time::sleep(Duration::from_millis(100)).await?;
	}
	```
- `fn() -> BoxedFuture`
	```rs
	it non_send "has non-send futures"{
		//example of a common non-send async function
		fantoccini::ClientBuilder::native().connect("http://example.com").await;
	}
	```

By default Sweet will detect the `await` keyword and mark that test as containing `Send` Futures. The `non_send` flag must be specified if your test contains a non-send future.

Note: currently if the runner finds any `non_send` tests it will run all tests on the main thread, even if the parallel flag is supplied.