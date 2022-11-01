# sweet

> *Very early stage warning! No ongoing maintenance is guaranteed*

Basically a [jest](https://jestjs.io/) clone, the `sweet` crate will set you up with a beautiful test harness and intuitive matchers that are easy on the eyes.

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
	rustup default nightly
	cargo test --test sweet
	```


## Features

- Nested Tests
	- Sweet is designed to collect and run all tests in one go. All tests exposed in the `sweet.rs` file will be run:


		```rust
		//test/sub_dir/some_test.rs
		sweet!{
			it "works" {
				expect(true).to_be_true();
			}
		}
		//test/sub_dir/mod.rs
		mod some_test
		//test/sweet.rs
		#![feature(imported_main)]
		pub use sweet::*;
		mod sub_dir;
		```
- Pretty success messages
	- ![success](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/success.png)
- In progress indication
	- ![progress](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/progress.png)
- Failure context
	- ![failure](https://raw.githubusercontent.com/mrchantey/forky/main/docs/images/failure.png)

## Example Commands

- Run 
   - `cargo test --test sweet`
- With watch
   - `cargo watch -q -x 'test --test sweet -- -w'`
   - Clears terminal on each run
   - Returns an exit code zero (cleaner output)
- Specify filename
   - `cargo test --test sweet -- my_test`

## Reference
- [jest](https://jestjs.io/)
- [demonstrate](https://crates.io/crates/demonstrate)
- [speculate](https://github.com/utkarshkukreti/speculate.rs)
	- Nicer syntax