use super::*;
use crate::*;
use anyhow::Result;
use forky_fs::*;
use std::time::Instant;

pub struct TestRunnerNative;

impl TestRunnerNative {
	#[tokio::main]
	pub async fn run() -> Result<()> {
		// dont exit program on panic?
		// let _ = std::panic::take_hook();

		let config = TestRunnerConfig::from_cli_args();
		if config.watch {
			terminal::clear()
		}

		let intro = TestRunner::pretty_print_intro(&config);
		println!("{intro}");

		let start_time = Instant::now();

		let collector = TestCollectorNative::new();
		// let results = collector.run(&config);
		let results = collector.run_parallel_maybe(&config).await;
		let duration = start_time.elapsed();
		let summary = TestRunner::pretty_print_summary(&results, duration);

		println!("{summary}");

		let no_tests = results.cases.tests == 0;
		if config.watch || no_tests {
			return Ok(());
		}
		terminal::show_cursor();

		match results.suites.failed {
			0 => Ok(()),
			1 => Err(anyhow::anyhow!(
				"{} test suite failed",
				results.suites.failed
			)),
			_ => Err(anyhow::anyhow!(
				"{} test suites failed",
				results.suites.failed
			)),
		}
	}
}


/*
Test Suites: 3 skipped, 42 passed, 42 of 45 total
Tests:       9 skipped, 109 passed, 118 total
Snapshots:   1 passed, 1 total
Time:        23.497 s
Ran all test suites.
*/
