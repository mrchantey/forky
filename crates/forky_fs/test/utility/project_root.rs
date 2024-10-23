use sweet::*;

#[sweet_test]
fn works() -> Result<()> {
	expect(true).to_be_true()?;

	Ok(())
}
