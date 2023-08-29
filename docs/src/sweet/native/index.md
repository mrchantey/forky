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
	  it "demonstrates sweet" {
			assert!(true == false);
			expect(true).to_be_false()?;
			expect("some string").not().to_start_with("some")?;
	  }
	}
	```
1. run `cargo run --example sweet`
1. optional - try changing the above matchers so the test passes ⚡

Here's an example output of a runner with a few tests:

![native-runner](../images/success.png)
