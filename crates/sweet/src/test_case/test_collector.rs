use super::*;
use std::collections::HashMap;


pub trait TestCollector<T>
where
	T: TestCase + Clone + Sized,
{
	fn suites(&self) -> &Vec<TestSuite<T>>;
	fn suites_to_run(&self, config: &TestRunnerConfig) -> Vec<&TestSuite<T>> {
		self.suites()
			.iter()
			.filter(|s| config.suite_passes_filter(s.file.as_str()))
			.collect::<Vec<_>>()
	}

	fn collect_cases() -> Vec<T>;

	fn collect_suites() -> Vec<TestSuite<T>> {
		let mut files: HashMap<String, TestSuite<T>> = HashMap::new();
		let cases = Self::collect_cases();
		for case in cases.iter() {
			// let case: &T = &case;
			if !files.contains_key(case.file()) {
				let file = String::from(case.file());
				files.insert(file.clone(), TestSuite::new(file));
			}
			files.get_mut(case.file()).unwrap().tests.push(case.clone());
		}

		let mut files = files.iter().map(|f| f.1.clone()).collect::<Vec<_>>();
		files.sort_by(|a, b| a.file.cmp(&b.file));

		for file in files.iter_mut() {
			file.contains_only = file
				.tests
				.iter()
				.any(|t| *t.config() == TestCaseConfig::Only);
		}

		files
	}

	fn run(&self, config: &TestRunnerConfig) -> ResultSummary {
		self.suites_to_run(config)
			.iter()
			.map(|s| s.run(config, TestSuite::<T>::run_strategy))
			.collect::<Vec<_>>()
			.into()
	}
}


pub trait TestCollectorParallel<T>: TestCollector<T>
where
	T: TestCase + Clone + Send + Sync,
{
	fn run_parallel(&self, config: &TestRunnerConfig) -> ResultSummary {
		if config.parallel {
			use rayon::prelude::*;
			self.suites_to_run(config)
				.par_iter()
				.map(|s| s.run(config, TestSuite::<T>::run_parallel_strategy))
				.collect::<Vec<_>>()
				.into()
		} else {
			self.run(config)
		}
	}
}
