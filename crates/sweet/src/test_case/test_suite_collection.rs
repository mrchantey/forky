use super::*;
use crate::TestSuiteResult;
use std::collections::HashMap;

// use futures::stream::FuturesUnordered;
// use futures::StreamExt;


pub struct TestSuiteCollection(pub Vec<TestFile>);

impl TestSuiteCollection {
	pub fn new() -> Self {
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

		Self(files)
	}


	pub fn run(&self, config: &TestRunnerConfig) -> Vec<TestSuiteResult> {
		//TODO parallel
		self.0
			.iter()
			.filter(|s| config.suite_passes_filter(s.file))
			.map(|s| s.run())
			.collect::<Vec<_>>()
	}

	#[cfg(not(target_arch = "wasm32"))]
	pub fn run_parallel(
		&self,
		config: &TestRunnerConfig,
	) -> Vec<TestSuiteResult> {
		self.0
			.iter()
			.filter(|s| config.suite_passes_filter(s.file))
			.map(|s| {
				let s = s.clone();
				std::thread::spawn(move || s.run())
			})
			.map(|h| h.join().unwrap())
			.collect::<Vec<_>>()
	}
}
