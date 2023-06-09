#![feature(imported_main)]
use anyhow::Result;
pub use sweet::*;

#[sweet_test]
fn foobar() -> Result<()> {
	println!("woah!");
	expect(true).to_be_true()?;
	Ok(())
}
#[sweet_test]
fn fizzboo() {
	println!("woah!");
	expect(true).to_be_true()?;
}
