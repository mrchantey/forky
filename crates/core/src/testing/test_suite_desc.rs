use super::TestSuite;

pub struct TestSuiteDesc {
	pub file: &'static str,
	pub name: &'static str,
	pub func: fn(suite: &mut TestSuite),
}

inventory::collect!(TestSuiteDesc);
