# Parallel & Async Tests

> NOT YET IMPLEMENTED

Internally each test will be stored as one of three types:

- `fn() -> Result<()>`
	```rs
	it "has no futures"{
		expect(true).to_be_true()?;
	}
	```
- `fn() -> BoxedFutureSend`
	- Send + Sync async test
	```rs
	it "has send futures"{
		visit("example.com").await?;
	}
	```
		visit("example.com").await?;
- `fn() -> BoxedFuture`
	- Non-send async test
	```rs
	it "has non-send futures"{
		fantoccini::ClientBuilder::native().connect("example.com").await
	}
	```

By default Sweet will run your tests asynchronously if required, but even if the parallel flag is not supplied it can't yet determine if the future is `Send` (for parallelism) or `UnwindSafe` (for catching panics).

An example of a non-parallelizable async function would be:
```rs
```

This would need to be flagged as `NoParallel` to compile:

```rs
sweet! {

	it NoParallel "connects"{
		let c = ClientBuilder::native()
			.connect("http://localhost:9515").await?;
	}

}
```