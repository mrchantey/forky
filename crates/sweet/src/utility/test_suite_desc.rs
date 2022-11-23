use super::TestSuite;
use anyhow::Result;
pub struct TestSuiteDesc {
	pub file: &'static str,
	pub name: &'static str,
	pub func: fn(suite: &mut TestSuite) -> Result<()>,
}

inventory::collect!(TestSuiteDesc);
