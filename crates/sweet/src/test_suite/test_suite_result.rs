use colorize::*;


#[derive(Debug, Default)]
pub struct ResultSummary {
	pub suites_arr: Vec<TestSuiteResult>,
	pub suites: TestSuiteResult,
	pub cases: TestSuiteResult,
}

impl Into<ResultSummary> for Vec<TestSuiteResult> {
	fn into(self) -> ResultSummary { ResultSummary::from_suites_arr(self) }
}

impl ResultSummary {
	fn from_suites_arr(suites_arr: Vec<TestSuiteResult>) -> Self {
		let mut suites = TestSuiteResult::new();
		let cases = suites_arr.iter().fold(
			TestSuiteResult::default(),
			|mut acc, item| {
				acc.tests += item.tests;
				acc.failed += item.failed;
				acc.skipped += item.skipped;

				suites.tests += 1;
				if item.failed > 0 {
					suites.failed += 1;
				}

				acc
			},
		);
		ResultSummary {
			suites_arr,
			suites,
			cases,
		}
	}
}

#[derive(Debug, Default)]
pub struct TestSuiteResult {
	pub tests: usize,
	pub failed: usize,
	pub skipped: usize,
}


impl TestSuiteResult {
	pub fn new() -> Self {
		TestSuiteResult {
			tests: 0,
			failed: 0,
			skipped: 0,
		}
	}
	pub fn pretty_print(&self, prefix: &'static str) -> String {
		let TestSuiteResult {
			tests,
			failed,
			skipped,
		} = self;
		let passed = tests - failed - skipped;
		let mut summaries: Vec<&str> = Vec::new();
		let passed_str = format!("{passed} passed").bold().green();
		let skipped_str = format!("{skipped} skipped").bold().yellow();
		let failed_str = format!("{failed} failed").bold().red();
		let total_str = if passed == *tests {
			format!("{tests} total")
		} else {
			format!("{passed} of {tests} total")
		};
		if *failed > 0 {
			summaries.push(&failed_str);
		}
		if *skipped > 0 {
			summaries.push(&skipped_str);
		}
		summaries.push(&passed_str);
		summaries.push(&total_str);
		format!("{}{}", prefix.bold(), summaries.join(", "))
	}
}
