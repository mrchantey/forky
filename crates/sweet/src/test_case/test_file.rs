use super::*;
use crate::TestLogger;
use crate::TestSuiteResult;
use std::collections::HashMap;

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

	pub fn collect() -> Vec<TestFile> {
		let mut files: HashMap<&'static str, TestFile> = HashMap::new();
		for case in inventory::iter::<TestCaseDesc> {
			let case: &TestCaseDesc = case;
			if !files.contains_key(case.file) {
				files.insert(case.file, TestFile::new(case.file));
			}
			files.get_mut(case.file).unwrap().tests.push(case.clone());
		}

		let mut files = files.iter().map(|f| f.1.clone()).collect::<Vec<_>>();
		files.sort_by(|a, b| a.file.cmp(&b.file));

		for file in files.iter_mut() {
			file.contains_only =
				file.tests.iter().any(|t| t.config == TestCaseConfig::Only);
		}

		files
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
