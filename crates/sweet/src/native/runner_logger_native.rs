use crate::test_runner::*;
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
		let summary = results.end_str(duration);
		println!("{summary}");
	}
}
