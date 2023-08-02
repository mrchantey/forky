# sweet
Write many tests quickly and cleanly.

> *Very early stage warning, do not use seriously!*

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
	```rust
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
2. add some wasm matchers to your test
	```rust
	//mount a div in a framework of your choice, we'll use leptos here :)
	mount(|cx|view!{cx,<h1>"This is a heading"</h1>});
	expect_el("h1")?.to_contain_text("This is a heading")?;
	```
3. run `forky sweet`
	- requires [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)

![wasm-runner](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/wasm-runner.png)

## Features

### Performance

Sweet produces a single binary for each crate. The default rust intergration test runner creates a seperate binary for each test, which ramps up compile times, see [this blog](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html) for more info.

The wasm runner 

### Args (native)
- filter by filename: `cargo run --example sweet -- some_dir/my_test`
- `-w` argument
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
let _handle = set_timeout(||{
	mount(|cx|view!{cx,<div>"hello world!"</div>});
},Duration::from_millis(100));

poll_ok(||expect_el("div")).await?
	.to_contain_text("hello world!")?;
```

### Informative outputs
- Long running tests show which suite is hanging
	- ![progress](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/progress.png)
- Failures are highly descriptive 
	- ![failure](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/failure.png)

## Misc

### Why use `[[example]]` instead of `[[test]]`
This makes it easier for the wasm test runner to produce cleaner output, but if you're only running native tests feel free to use `[[test]]` with `harness=false`.

### What about wasm-bindgen-test?
Sweet has different priorities from [wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) in its current state.
- Interactive - the runner will list all tests and they can be run at-will in the browser.
- UI - Tests are run in a *mostly* isolated iframe (see TODO)

### TODO
- wasm
	- node & headless support
	- seperate interactive runner from tests, currently the runner code, css etc is included.

### Reference
- Matchers inspired by [jest](https://jestjs.io/)
- WASM runner inspired by [cypress](https://www.cypress.io/)

