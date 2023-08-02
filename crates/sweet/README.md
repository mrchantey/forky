# sweet

Write many tests quickly and cleanly.

## Features

- 🔥 Parallel
- 🕙 Async
- 🌍 WASM UI tests
- ☮️ Intuitive matchers
- 🌈 Pretty output

```rust
pub use sweet::*;

sweet! {
  it "works" {
		// use regular assertions
		assert!(true == false);
		// or pretty matchers
		expect(true).to_be_false()?;
		expect("some string").not().to_start_with("some")?;
  }
}
```

# Native

The Sweet runner has a couple of advantages over [default tests](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html).
- Suites - Organize your tests into collections
- Matchers - Matchers specific to a type enables a runner to output more intuitive results instead of an opaque `panic!`
	- ```rs
		expect("foo").not().to_start_with("bar")
		//expected: NOT to start with 'bar'
		//received: 'foo'
		```
- Single Binary - The default intergration test approach creates a seperate binary for each test, which ramps up compile times, see [this blog](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html) for more info.

## Quickstart

1. edit `cargo.toml`
	```toml
	[dev-dependencies]
	sweet = # current version here

	[[example]]
	name = "sweet"
	path = "test/sweet.rs"
	```
2. create file `test/sweet.rs`
	```rust
	#![feature(imported_main)]
	pub use sweet::*;

	sweet! {
	  it "works" {
	    expect(true).to_be_false()?;
	  }
	}
	```
3. run `cargo run --example sweet`

note: we're using `[[example]]` here for compatability with the wasm test runner, but feel free to use `[[test]]` with `harness=false` if only running native tests
## Features - CLI
- Run 
	- `cargo run --example sweet`
- With watch
	- `cargo watch -q -x 'run --example sweet -- -w'`
	- Clears terminal on each run
	- Returns an exit code zero (cleaner output)
- Specify filename
	- `cargo run --example sweet -- my_test`
	- Use forward-slash `/` to specify directories
		- `cargo run --example sweet -- my_dir/my_test`

## WASM

The wasm test harness has different priorities from [wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)
- UI - Tests are run in a **mostly* isolated iframe (see TODO)
- Interactive - the runner will list all tests and they can be run at-will in the browser.

### Quickstart

1. Follow native quickstart
2. install the cli
   - `cargo install forky_cli`
3. `forky_cli sweet`
   - or for workspaces `forky_cli sweet -p my_package`

## Features - Summary
- Pretty Messages
	- Success
		- ![success](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/success.png)
	- In progress
		- ![progress](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/progress.png)
	- Failure
		- ![failure](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/failure.png)

## Dont Panic

Or do, thats ok too. Currently you'll get the prettiest output by using the provided matchers that return results intstead of panicing, *especially* in wasm as `panic=unwind` isnt yet supported for wasm.

## Reference
- Matchers inspired by [jest](https://jestjs.io/)
- WASM runner inspired by [cypress](https://www.cypress.io/)

## TODO
- wasm
	- node & headless support
	- seperate interactive runner from tests, currently the runner code, css etc is included.
	- catch panics in test, like how wasm-bindgen-test [does it](https://github.com/rustwasm/wasm-bindgen/blob/74bfc1f85ead6a3e0c37a86e5f93df3e692e217a/crates/test/src/rt/mod.rs#L227-L240)