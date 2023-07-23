# sweet

*Write many tests quickly and cleanly.*

## Features

- :fire:  Parallel - Test cases are run in parallel by default
- :earth_africa: WASM - Run tests in-browser with an interactive harness
- :rainbow: Pretty - Jest inspired matchers and outputs



```rust
pub use sweet::*;

sweet! {
  it "works" {
    assert!("regular assertions and panics".len() > 0);

    expect(true).to_be_false()?;
    expect("string matchers").to_contain("this string")?;
  }
}
```

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
2. run these commands
	```sh
	cargo test --test sweet
	```


## Features - Summary
- Pretty Messages
	- Success
		- ![success](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/success.png)
	- In progress
		- ![progress](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/progress.png)
	- Failure
		- ![failure](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/failure.png)

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


## Reference
- [jest](https://jestjs.io/)
- [demonstrate](https://crates.io/crates/demonstrate)
- [speculate](https://github.com/utkarshkukreti/speculate.rs)


## TODO
