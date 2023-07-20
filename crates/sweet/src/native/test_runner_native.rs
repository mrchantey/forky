use crate::*;
use anyhow::Result;
use colorize::*;
use forky_core::*;
use forky_fs::*;
use std::time::Instant;

pub struct TestRunnerNative;

impl TestRunnerNative {
	pub fn run() -> Result<()> {
		let config = TestRunnerConfig::from_cli_args();
		if config.watch {
			terminal::clear()
		}

		println!("\n🤘 sweet as! 🤘\n");
		if config.files.len() > 0 {
			println!("matching: {}\n", config.files.to_string());
		}

		let start_time = Instant::now();
		let mut results_suites = TestSuiteResult::new();

		let suites = TestSuiteCollection::new();
		let results_cases_arr = suites.run_parallel(&config);

		let results_cases = results_cases_arr.iter().fold(
			TestSuiteResult::default(),
			|mut acc, item| {
				acc.tests += item.tests;
				acc.failed += item.failed;
				acc.skipped += item.skipped;

				results_suites.tests += 1;
				if item.failed > 0 {
					results_suites.failed += 1;
				}

				acc
			},
		);

		print!("\n");
		if results_cases.tests == 0 {
			println!("{}", String::from("No Tests Found\n").red());
			return Ok(());
		} else if results_cases.failed == 0 {
			println!("{}", "All tests passed\n".bold().cyan().underlined());
		}

		results_suites.log_summary("Test Suites:\t");
		results_cases.log_summary("Tests:\t\t");
		TestLogger::log_time(start_time.elapsed());

		if config.watch {
			return Ok(());
		}
		terminal::show_cursor();

		match results_suites.failed {
			0 => Ok(()),
			1 => Err(anyhow::anyhow!(
				"{} test suite failed",
				results_suites.failed
			)),
			_ => Err(anyhow::anyhow!(
				"{} test suites failed",
				results_suites.failed
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
