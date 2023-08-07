use crate::*;

#[derive(Default, Debug, Clone)]
pub struct TestSuiteNative {
	pub file: String,
	pub tests: Vec<TestCaseNative>,
	pub config: TestSuiteConfig,
}


impl TestSuiteTrait<TestCaseNative> for TestSuiteNative {
	fn new(file: String) -> Self {
		Self {
			file,
			tests: Vec::new(),
			config: TestSuiteConfig::default(),
		}
	}
	fn file(&self) -> &str { self.file.as_str() }
	fn config(&self) -> &TestSuiteConfig { &self.config }
	fn tests(&self) -> &Vec<TestCaseNative> { &self.tests }
	fn tests_mut(&mut self) -> &mut Vec<TestCaseNative> { &mut self.tests }

	async fn run_cases(
		to_run: Vec<&TestCaseNative>,
		config: &TestRunnerConfig,
	) -> Vec<anyhow::Error> {
		if config.parallel {
			run_cases_parallel(to_run, config).await
		} else {
			run_cases_series(to_run, config).await
		}
	}
}
