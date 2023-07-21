#![feature(imported_main)]
use anyhow::Result;
use forky_core::*;
pub use sweet::*;

#[sweet_test]
fn passes() -> Result<()> {
	expect(true).to_be_false()?;
	Ok(())
}
#[sweet_test]
fn fails() -> Result<()> {
	expect(true).to_be_true()?;
	Ok(())
}
