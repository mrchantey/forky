use anyhow::Result;

pub struct TestCaseDesc {
	pub file: &'static str,
	pub name: &'static str,
	pub func: fn() -> Result<()>,
}

inventory::collect!(TestCaseDesc);
