use crate::*;

#[derive(Default, Debug, Clone)]
pub struct TestSuiteWasm {
	pub file: String,
	pub tests: Vec<TestCaseWasm>,
	pub config: TestSuiteConfig,
}


impl TestSuiteTrait<TestCaseWasm> for TestSuiteWasm {
	fn new(file: String) -> Self {
		Self {
			file,
			tests: Vec::new(),
			config: TestSuiteConfig::default(),
		}
	}
	fn file(&self) -> &str { self.file.as_str() }
	fn config(&self) -> &TestSuiteConfig { &self.config }
	fn tests(&self) -> &Vec<TestCaseWasm> { &self.tests }
	fn tests_mut(&mut self) -> &mut Vec<TestCaseWasm> { &mut self.tests }
}
