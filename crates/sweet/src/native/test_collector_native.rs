use super::*;
use crate::*;

pub struct TestCollectorNative(pub Vec<TestSuiteNative>);

impl TestCollectorNative {
	pub fn new() -> Self { Self(Self::collect_cases_to_suites()) }
}

impl TestCollectorParallel<TestCaseNative, SuiteLoggerNoop, TestSuiteNative>
	for TestCollectorNative
{
}

impl TestCollector<TestCaseNative, SuiteLoggerNoop, TestSuiteNative>
	for TestCollectorNative
{
	fn suites(&self) -> &Vec<TestSuiteNative> { &self.0 }

	fn collect_cases() -> Vec<TestCaseNative> {
		let mut cases = Vec::new();
		for case in inventory::iter::<TestCaseNative> {
			let case: &TestCaseNative = case;
			cases.push(case.clone());
		}
		cases
	}
}
