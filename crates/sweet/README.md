# sweet

*Write many tests quickly and cleanly.*

## Features

- 🔥 Parallel
- 🕙 Async
- 🌍 WASM
- ☮️ Intuitive matchers
- 🌈 Pretty output



```rust
pub use sweet::*;

sweet! {
  it "works" {
		//use regular assertions
		assert!(true == false);
		//or matchers
		expect(true).to_be_false()?;
		expect("some string").not().to_start_with("some")?;
  }
}
```

# Native

The Sweet harness has two main adavantages over [default tests](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html).
- Suites - Organize your tests into collections
- Matchers - the `expect` approach provides intuitive matchers and pretty outputs

## Quickstart

1. edit `cargo.toml`
	```toml
	[dev-dependencies]
	sweet = # current version here

	[[test]]
	name = "sweet"
	path = "test/sweet.rs"
	harness = false
	```
1. create file `test/sweet.rs`
	```rust
	#![feature(imported_main)]
	pub use sweet::*;

	sweet! {
	  it "works" {
	    expect(true).to_be_false()?;
	  }
	}
	```
2. run `cargo test --test sweet`


## Features - CLI
- Run 
	- `cargo test --test sweet`
- With watch
	- `cargo watch -q -x 'test --test sweet -- -w'`
	- Clears terminal on each run
	- Returns an exit code zero (cleaner output)
- Specify filename
	- `cargo test --test sweet -- my_test`
	- Use forward-slash `/` to specify directories
		- `cargo test --test sweet -- my_dir/my_test`

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
- seperate interactive wasm runner from tests, currently the runner code, css etc is included.
- catch wasm panics in test, like how wasm-bindgen-test [does it](https://github.com/rustwasm/wasm-bindgen/blob/74bfc1f85ead6a3e0c37a86e5f93df3e692e217a/crates/test/src/rt/mod.rs#L227-L240)