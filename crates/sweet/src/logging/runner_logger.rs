use colorize::*;
use std::time::Duration;
use forky_core::*;
use crate::*;

pub struct RunnerLogger;

impl RunnerLogger {

	pub fn log_intro(config: &TestRunnerConfig) {
		let intro = Self::pretty_print_intro(&config);
		log!("{intro}");
	}

	fn pretty_print_intro(config: &TestRunnerConfig) -> String {
		format!("\n🤘 sweet as! 🤘\n\n{config}")
	}

	pub fn log_summary(
		results: &ResultSummary,
		duration: Duration,
	) {
		let summary = Self::pretty_print_summary(&results, duration);
		log!("{summary}");
	}

	fn pretty_print_summary(
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
