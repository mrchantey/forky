use crate::*;
use forky_core::*;
use forky_web::*;
use std::time::Duration;
use web_sys::console;

pub struct RunnerLoggerWasm {
	start_time: f64,
	logger_export: LoggerExport,
}


impl RunnerLogger for RunnerLoggerWasm {
	fn start(config: &TestRunnerConfig) -> Self {
		console::clear();
		let intro = Self::pretty_print_intro(&config);
		log!("{intro}");
		let logger_export = LoggerExport::init();
		logger_export.push(&intro);
		let start_time = performance_now();
		Self {
			start_time,
			logger_export,
		}
	}
	fn end(self, results: &TestRunnerResult) {
		let duration =
			Duration::from_millis((performance_now() - self.start_time) as u64);
		let summary = Self::pretty_print_summary(&results, duration);
		log!("{summary}");
		self.logger_export.push(&summary);
		self.logger_export.push("_sweet_end");
	}
}
