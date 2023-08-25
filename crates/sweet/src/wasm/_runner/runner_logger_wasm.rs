use crate::*;
use forky_core::*;
use forky_web::*;
use std::time::Duration;
use web_sys::console;

pub struct RunnerLoggerWasm {
	start_time: f64,
}


impl RunnerLogger for RunnerLoggerWasm {
	fn start(config: &TestRunnerConfig) -> Self {
		console::clear();
		let intro = Self::pretty_print_intro(&config);
		log!("{intro}");
		let start_time = performance_now();
		Self {
			start_time,
		}
	}
	fn end(self, results: &TestRunnerResult) {
		let duration =
			Duration::from_millis((performance_now() - self.start_time) as u64);
		let summary = Self::pretty_print_summary(&results, duration);
		log!("{summary}");
	}
}
