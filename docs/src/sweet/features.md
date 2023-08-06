# Features

## Macros

```rs
sweet!{
	it "works"{}
	test "is an alias for it"{}
	it skip "wont run"{}
	it only "will exclude non-onlys in this suite"{}
}
```
## Performance

- Single Binary - The default rust intergration test runner creates a seperate binary for each test, which ramps up compile times, see [this blog](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html) for more info.
- Very Hot Reload - The wasm cli tool features a lightweight dev server that uses `tower-livereload` instead of shutting down and restarting.

## Matchers
Instead of an opaque `panic!`, matchers provide the runner with enough info to produce a highly descriptive failure:
```rs
expect("foobar").not().to_start_with("foo")?;
/*
Expected: NOT to start with 'foo'
Received: foobar
*/
```

## Async Matchers
Lots of web stuff happens at weird times, so we've got helpers like `poll_ok`, which will wait for 4 seconds before failing.

```rs
let _handle = set_timeout(|| {
		mount(|cx| view!{cx,
			<div>"sweet as!"</div>
		});
	}, Duration::from_millis(100));

poll_ok(|| expect_el("div")).await?
	.to_contain_text("sweet as!")?;
```

## Informative outputs
- Long running tests show which suite is hanging
	- ![progress](images/progress.png)
- Failures are highly descriptive 
	- ![failure](images/failure.png)
