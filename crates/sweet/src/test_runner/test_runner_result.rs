use super::*;
use crate::test_suite::*;
use colorize::*;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Default, Serialize, Deserialize)]
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
	pub fn did_fail(&self) -> bool { self.cases.failed > 0 }

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


	pub fn end_str(&self, duration: Duration) -> String {
		let mut post_run = String::from("\n");

		if self.cases.tests == 0 {
			post_run += "No Tests Found\n".red().as_str();
			return post_run;
		} else if self.cases.failed == 0 {
			post_run +=
				"All tests passed\n".bold().cyan().underlined().as_str();
		}

		post_run += self.suites.pretty_print("Suites:\t\t").as_str();
		post_run.push('\n');
		post_run += self.cases.pretty_print("Tests:\t\t").as_str();
		post_run.push('\n');
		post_run += Self::print_time(duration).as_str();
		post_run
	}

	fn print_time(duration: Duration) -> String {
		let millis = duration.as_millis();
		let prefix = "Time:\t\t".bold();
		if millis < 100 {
			format!("{}{} ms\n\n", prefix, millis)
		} else {
			format!("{}{:.2} s\n\n", prefix, millis as f32 * 0.001)
		}
	}
}
