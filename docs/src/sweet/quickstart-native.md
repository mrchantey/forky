# Quickstart - Native


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
			expect("some string").not().to_start_with("foo")?;
	  }
	}
	```
1. run `cargo run --example sweet`


Here's an example output of a runner with a few tests:

![native-runner](images/success.png)

## Passing arguments

The binary is isself a mini cli with options, to see them all try:

`cargo run --example sweet --help`

```
Native runner for your tests.

Usage: sweet_sweet.exe [OPTIONS] [filter]...

Arguments:
  [filter]...  filter by directory or path name, ie. `/test1.rs /e2e/`

Options:
  -w, --watch     clears screen and does not return error
  -p, --parallel  run tests in parallel
```