# Parallel & Async Tests

> NOT YET IMPLEMENTED

By default Sweet will run your tests asynchronously if required, but even if the parallel flag is not supplied it can't yet determine if the future is `Send` (for parallelism) or `UnwindSafe` (for catching panics).

An example of a non-parallelizable async function would be:
```rs
	fantoccini::ClientBuilder::native().connect()
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