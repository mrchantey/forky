use forky_core::PathExt;
use std::path::Path;
use std::path::PathBuf;
use sweet::*;


struct Case;
impl TestCase for Case {
	fn name(&self) -> &'static str { "works" }
	fn config(&self) -> &TestCaseConfig { todo!() }
	fn path(&self) -> PathBuf { Path::new(file!()).to_forward_slash() }
	async fn run_func(&self) -> Result<()> { Ok(()) }
}

#[sweet_test]
fn sweet_test_macro_compiles() -> Result<()> { Ok(()) }

#[sweet_test(skip)]
fn skips() -> Result<()> { expect(true).to_be_false() }

#[sweet_test(skip, only)]
fn skips_only() -> Result<()> { expect(true).to_be_false() }

#[sweet_test(e2e)]
fn e2e() -> Result<()> { Ok(()) }

#[cfg(not(target_arch = "wasm32"))]
#[sweet_test(e2e, non_send)]
async fn can_be_async() -> Result<()> {
	tokio::time::sleep(std::time::Duration::from_millis(1)).await;
	Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
#[sweet_test(non_send)]
async fn can_be_async_non_send() -> Result<()> {
	tokio::time::sleep(std::time::Duration::from_millis(1)).await;
	Ok(())
}



sweet! {
	it "works" {

		let case = Case;
		expect(case.path().to_str().unwrap())
			.to_be("crates/sweet/test/common/test_case.rs")?;
	}
}
