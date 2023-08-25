use crate::*;


#[derive(Debug, Default)]
pub struct TestRunnerResult {
	// pub suites_arr: Vec<TestSuiteResult>,
	pub suites: TestSuiteResult,
	pub cases: TestSuiteResult,//TODO probably newtype this
}

impl Into<TestRunnerResult> for Vec<TestSuiteResult> {
	fn into(self) -> TestRunnerResult {
		TestRunnerResult::from_suites_arr(self)
	}
}

impl TestRunnerResult {
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
		TestRunnerResult {
			// suites_arr,
			suites,
			cases,
		}
	}
}