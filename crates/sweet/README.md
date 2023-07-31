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
		//or
		expect(true).to_be_false()?;
  }
}
```

# Native

Sweet supports running tests natively.

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

### Quickstart

1. Follow quickstart ^
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
- prettier wasm panics
- seperate wasm runner from tests, currently css etc is all bundled in.