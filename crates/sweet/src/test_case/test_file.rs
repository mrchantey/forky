use super::*;
use crate::TestLogger;
use crate::TestSuiteResult;

#[derive(Default, Debug, Clone)]
pub struct TestFile {
	pub file: &'static str,
	pub tests: Vec<TestCaseDesc>,
	pub contains_only: bool,
	pub config: TestCaseConfig,
}


impl TestFile {
	pub fn new(file: &'static str) -> Self {
		Self {
			file,
			tests: Vec::new(),
			contains_only: false,
			config: TestCaseConfig::Default,
		}
	}

	fn should_skip(&self, test: &TestCaseDesc) -> bool {
		test.config == TestCaseConfig::Skip
			|| self.config == TestCaseConfig::Skip
			|| (self.contains_only && test.config != TestCaseConfig::Only)
	}

	pub fn run(&self) -> TestSuiteResult {
		let mut result = TestSuiteResult::default();
		let mut logger = TestLogger::start(self.file);
		result.tests = self.tests.len();

		for test in self.tests.iter() {
			if self.should_skip(test) {
				result.skipped += 1;
				continue;
			}
			if let Err(msg) = test.run() {
				result.failed += 1;
				logger.log.push_str(&msg.to_string());
			}
		}
		logger.end(&result);

		result
	}
}
