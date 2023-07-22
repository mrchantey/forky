use super::*;
use crate::*;
use std::collections::HashMap;

type TestSuiteNative = TestSuite<TestCaseNative>;

pub struct TestCollectorNative(pub Vec<TestSuiteNative>);

impl TestCollectorNative {
	pub fn new() -> Self {
		let mut files: HashMap<&'static str, TestSuiteNative> = HashMap::new();
		for case in inventory::iter::<TestCaseNative> {
			let case: &TestCaseNative = case;
			if !files.contains_key(case.file) {
				files.insert(case.file, TestSuite::new(case.file));
			}
			files.get_mut(case.file).unwrap().tests.push(case.clone());
		}

		let mut files = files.iter().map(|f| f.1.clone()).collect::<Vec<_>>();
		files.sort_by(|a, b| a.file.cmp(&b.file));

		for file in files.iter_mut() {
			file.contains_only =
				file.tests.iter().any(|t| t.config == TestCaseConfig::Only);
		}

		Self(files)
	}


	pub fn run(&self, config: &TestRunnerConfig) -> Vec<TestSuiteResult> {
		let to_run = self
			.0
			.iter()
			.filter(|s| config.suite_passes_filter(s.file))
			.collect::<Vec<_>>();
		if config.parallel {
			use rayon::prelude::*;
			to_run.par_iter().map(|s| s.run(config)).collect::<Vec<_>>()
		} else {
			to_run.iter().map(|s| s.run(config)).collect::<Vec<_>>()
		}
	}
}
