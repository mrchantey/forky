use crate::*;
use std::time::Instant;

pub struct RunnerLoggerNative {
	start_time: Instant,
}
impl RunnerLogger for RunnerLoggerNative {
	fn start(config: &TestRunnerConfig) -> Self {
		let start_time = Instant::now();
		let intro = Self::pretty_print_intro(&config);
		println!("{intro}");
		Self { start_time }
	}
	fn end(self, results: &TestRunnerResult) {
		let duration = self.start_time.elapsed();
		let summary = Self::pretty_print_summary(&results, duration);
		println!("{summary}");
	}
}
