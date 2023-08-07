use crate::*;



#[derive(Default, Debug, Clone)]
pub struct TestSuiteNative {
	pub file: &'static str,
	pub tests: Vec<TestCaseNative>,
	pub config: TestSuiteConfig,
}


impl TestSuiteTrait<TestCaseNative> for TestSuiteNative {
	fn file(&self) -> &str { self.file }
	fn config(&self) -> &TestSuiteConfig { &self.config }
	fn tests(&self) -> &Vec<TestCaseNative> { &self.tests }
}
