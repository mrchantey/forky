use super::*;
use colorize::*;
use forky_core::*;
use std::time::Duration;

pub struct TestRunner;

impl TestRunner {
	pub fn pretty_print_intro(config: &TestRunnerConfig) -> String {
		let mut pre_run = String::from("\n🤘 sweet as! 🤘\n");

		if config.files.len() > 0 {
			pre_run +=
				format!("\nmatching: {}\n", config.files.to_string()).as_str();
		}

		pre_run
	}

	pub fn pretty_print_summary(
		results: &ResultSummary,
		duration: Duration,
	) -> String {
		let mut post_run = String::from("\n");

		if results.cases.tests == 0 {
			post_run += "No Tests Found\n".red().as_str();
			return post_run;
		} else if results.cases.failed == 0 {
			post_run +=
				"All tests passed\n".bold().cyan().underlined().as_str();
		}

		post_run += results.suites.pretty_print("Suites:\t\t").as_str();
		post_run.push('\n');
		post_run += results.cases.pretty_print("Tests:\t\t").as_str();
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
