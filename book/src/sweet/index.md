# Sweet

> *Very early stage warning:*
> - breaking changes on patch versions
> - continued development not guaranteed
> - lots of unwraps, if you get stuck please create an issue and I'll have a look

## Features

- 🔥 Parallel
- 🕙 Async
- 🌍 WASM UI tests
- ☮️ Intuitive matchers
- 🌈 Pretty output
## Quickstart - Native

1. edit `cargo.toml`
	```toml
	[dev-dependencies]
	sweet = # current version here

	[[example]]
	name = "sweet"
	path = "test/sweet.rs"
	```
1. create file `test/sweet.rs`
	```rs
	#![feature(imported_main)]
	pub use sweet::*;

	sweet! {
	  it "works" {
			// use assertions
			assert!(true == false);
			// or pretty matchers
			expect(true).to_be_false()?;
			expect("some string").not().to_start_with("some")?;
	  }
	}
	```
1. run `cargo run --example sweet`

## Quickstart - WASM

1. Follow native quickstart
1. install the helper cli: `cargo install forky_cli`
2. mount some html and add some wasm matchers to your test
	```rs
	web_sys::window().unwrap()
			.document().unwrap()
			.body().unwrap()
			.set_inner_html("<h1>This is a heading</h1>");
	expect_el("h1")?.to_contain_text("This is a heading")?;
	```
3. run `forky sweet`
	- requires [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)

![wasm-runner](images/wasm-runner.png)

## Features

### Macros

```rs
sweet!{
	it "works"{}
	test "is an alias for it"{}
	it skip "wont run"{}
	it only "will exclude non-onlys in this suite"{}
}
```
### Performance

- Single Binary - The default rust intergration test runner creates a seperate binary for each test, which ramps up compile times, see [this blog](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html) for more info.
- Very Hot Reload - The wasm cli tool features a lightweight dev server that uses `tower-livereload` instead of shutting down and restarting.

### Args (native)
- filter by filename: `cargo run --example sweet -- some_dir/my_test`
- watch mode `-w`
	- `cargo watch -q -x 'run --example sweet -- -w'`
	- Clears terminal on each run
	- Returns an exit code zero (cleaner output)

### Matchers
Instead of an opaque `panic!`, matchers provide the runner with enough info to produce a highly descriptive failure:
```rs
expect("foobar").not().to_start_with("foo")?;
/*
Expected: NOT to start with 'foo'
Received: foobar
*/
```

### Async Matchers
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

### Informative outputs
- Long running tests show which suite is hanging
	- ![progress](images/progress.png)
- Failures are highly descriptive 
	- ![failure](images/failure.png)

## Misc

### Why use `[[example]]` instead of `[[test]]`
This makes it easier for the wasm test runner to produce cleaner output, but if you're only running native tests feel free to use `[[test]]` with `harness=false`.

### What about wasm-bindgen-test?
Sweet has different priorities from [wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) in its current state, namely the focus is on UI & interactivity.

### TODO
- wasm
	- cli - allow user to include static files in build step, ie css
	- node & headless support?
	- tests return Result<(),JsValue>?
- native
	- use clap & expose parallel flag

### Reference
- Matchers inspired by [jest](https://jestjs.io/)
- WASM runner inspired by [cypress](https://www.cypress.io/)

