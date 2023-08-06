#[derive(Debug, Default, Clone, PartialEq)]
pub struct TestSuiteConfig {
	pub skip: bool,
	pub only: bool,
	pub context: TestSuiteContext,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TestSuiteContext {
	#[default]
	Unit,
	EndToEnd,
}
