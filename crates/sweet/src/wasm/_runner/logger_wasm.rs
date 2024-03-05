use crate::test_runner::*;
use crate::test_suite::SuiteLogger;
use forky_web::*;
use std::time::Duration;
use wasm_bindgen::JsValue;
use web_sys::console;

pub struct RunnerLoggerWasm {
	start_time: f64,
}


impl RunnerLogger for RunnerLoggerWasm {
	fn start(config: &TestRunnerConfig) -> Self {
		console::clear();
		let intro = Self::pretty_print_intro(&config);
		log_web(&intro);
		let start_time = performance_now();
		Self { start_time }
	}
	fn end(self, results: &TestRunnerResult) {
		let duration =
			Duration::from_millis((performance_now() - self.start_time) as u64);
		let summary = results.end_str(duration);
		log_web(&summary);
	}
}

pub fn log_web(val: &str) { web_sys::console::log_1(&JsValue::from_str(val)); }

#[derive(Default, Debug, Clone)]
pub struct SuiteLoggerWasm;


impl SuiteLogger for SuiteLoggerWasm {
	fn on_start(_: String) -> Self { Self }
	fn on_end(self, end_str: String) { log_web(&end_str); }
}
