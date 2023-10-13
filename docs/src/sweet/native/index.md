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

	#[sweet_test]
	fn it_works() -> Result<()>{
		assert!(true == true);
		expect(true).to_be_true()?;
		expect("foobar")
			.not()
			.to_start_with("bazz")?;
		Ok(())
	}
	```
1. run `cargo run --example sweet`
1. optional - try changing the above matchers so the test fails ⚡

As an example here is the output of a runner with a few tests:

![native-runner](../images/success.png)
