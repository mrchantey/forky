use super::*;
use crate::*;

type TestSuiteNative = TestSuite<TestCaseNative>;

pub struct TestCollectorNative(pub Vec<TestSuiteNative>);

impl TestCollectorNative {
	pub fn new() -> Self { Self(Self::collect_suites()) }
}

impl TestCollectorParallel<TestCaseNative, SuiteLoggerNative>
	for TestCollectorNative
{
}

impl TestCollector<TestCaseNative, SuiteLoggerNative> for TestCollectorNative {
	fn suites(&self) -> &Vec<TestSuite<TestCaseNative>> { &self.0 }

	fn collect_cases() -> Vec<TestCaseNative> {
		let mut cases = Vec::new();
		for case in inventory::iter::<TestCaseNative> {
			let case: &TestCaseNative = case;
			cases.push(case.clone());
		}
		cases
	}
}
