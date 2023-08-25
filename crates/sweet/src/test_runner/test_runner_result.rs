use crate::*;


#[derive(Debug, Default)]
pub struct TestRunnerResult {
	pub suite_results: Vec<SuiteResult>,
	pub suites: ResultCount,
	pub cases: ResultCount, //TODO probably newtype this
}

impl Into<TestRunnerResult> for Vec<SuiteResult> {
	fn into(self) -> TestRunnerResult {
		TestRunnerResult::from_suite_results(self)
	}
}

impl TestRunnerResult {
	fn from_suite_results(suite_results: Vec<SuiteResult>) -> Self {
		let mut suites = ResultCount::new();
		let cases = suite_results.iter().fold(
			ResultCount::default(),
			|mut acc, item| {
				acc.tests += item.tests;
				acc.failed += item.failed.len();
				acc.skipped += item.skipped;

				suites.tests += 1;
				if item.failed.len() > 0 {
					suites.failed += 1;
				}

				acc
			},
		);
		TestRunnerResult {
			suite_results,
			suites,
			cases,
		}
	}
}
