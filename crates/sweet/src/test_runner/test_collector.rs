use super::*;
use crate::test_case::*;
use crate::test_suite::*;
use std::collections::HashMap;
use std::path::PathBuf;


pub trait TestCollector<Case, Suite>
where
	Case: TestCase + Clone + Sized,
	Suite: TestSuiteTrait<Case>,
{
	fn suites(&self) -> &Vec<Suite>;
	fn suites_to_run(&self, config: &TestRunnerConfig) -> Vec<&Suite> {
		self.suites()
			.iter()
			.filter(|s| config.suite_passes_filter(s.file()))
			.collect::<Vec<_>>()
	}

	fn collect_cases() -> Vec<Case>;

	fn collect_cases_to_suites() -> Vec<Suite> {
		let mut suites: HashMap<PathBuf, Suite> = HashMap::new();
		let cases = Self::collect_cases();
		for case in cases.iter() {
			let path = case.path();
			if !suites.contains_key(&path) {
				suites.insert(path.clone(), Suite::new(path.clone()));
			}
			suites.get_mut(&path).unwrap().push_test(case.clone());
		}
		let mut suites2 = Vec::with_capacity(suites.len());
		for (_, suite) in suites {
			suites2.push(suite);
		}
		// let mut suites = suites.iter().collect::<Vec<Suite>>();
		suites2.sort_by(|a, b| a.file().cmp(&b.file()));
		suites2
	}
}
