pub use sweet::*;

#[sweet_test]
fn assert_test() { assert!(true) }
#[sweet_test]
fn result_test() -> anyhow::Result<()> {
	expect(true).to_be_true()?;
	Ok(())
}
#[sweet_test]
async fn async_assert_test() { assert!(true) }
#[sweet_test]
async fn async_result_test() -> anyhow::Result<()> {
	expect(true).to_be_true()?;
	Ok(())
}
