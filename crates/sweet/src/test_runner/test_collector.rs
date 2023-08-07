use crate::*;
use std::collections::HashMap;


pub trait TestCollector<Case, Logger, Suite>
where
	Case: TestCase + Clone + Sized,
	Logger: SuiteLogger,
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
		let mut suites: HashMap<String, Suite> = HashMap::new();
		let cases = Self::collect_cases();
		for case in cases.iter() {
			let path = case.path();
			let path = path.to_str().unwrap();
			if !suites.contains_key(path) {
				let file = String::from(path);
				suites.insert(file.clone(), Suite::new(file));
			}
			suites.get_mut(path).unwrap().push_test(case.clone());
		}
		let mut suites2 = Vec::with_capacity(suites.len());
		for (_, suite) in suites {
			suites2.push(suite);
		}
		// let mut suites = suites.iter().collect::<Vec<Suite>>();
		suites2.sort_by(|a, b| a.file().cmp(&b.file()));
		suites2
	}

	async fn run(&self, config: &TestRunnerConfig) -> ResultSummary {
		let to_run = self.suites_to_run(config);
		let mut results = Vec::with_capacity(to_run.len());
		for suite in to_run {
			let result = suite
				.run::<Logger>(config)
				.await;
			results.push(result);
		}
		results.into()
		// self.suites_to_run(config)
		// 	.iter()
		// 	.map(move |s| s.run::<Logger, TestSuiteRunnerSeries<Case>>(config))
		// 	.collect::<Vec<_>>()
		// 	.into()
	}
}
