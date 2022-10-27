use crate::*;
use colorize::*;
use crossterm::terminal;
use forky_core::{utility, *};
use std::path::PathBuf;
use std::slice::Iter;
use std::time::Instant;


fn suite_in_args(path: &str, args: &Vec<String>) -> bool { args.iter().any(|a| path.contains(a)) }
fn vec_contains_str(path: &str, args: &Vec<String>) -> bool { args.iter().any(|a| a == path) }
fn arr_contains_str(path: &str, arr: &[&str]) -> bool { arr.iter().any(|a| *a == path) }

const FLAGS: &'static [&str] = &["-w"];

struct Args {
	watch: bool,
	files: Vec<String>,
}

pub struct TestRunner {}


impl TestRunner {
	pub fn run() -> Result<(), MatcherError> {
		let args = parse_args();
		if args.watch {
			utility::Terminal::clear()
		}
		log!("\n🤘 lets get forky! 🤘\n");

		let start_time = Instant::now();
		let mut suite_results: Vec<TestSuiteResult> = Vec::new();
		let mut desc_arr: Vec<&TestSuiteDesc> = Vec::new();
		for t in inventory::iter::<TestSuiteDesc> {
			desc_arr.push(t);
		}
		desc_arr.sort_by(|a, b| a.file.cmp(&b.file));

		for t in desc_arr {
			if args.files.len() > 0 && !suite_in_args(t.file, &args.files) {
				continue;
			}
			let mut suite = TestSuite::new(t);
			suite.print_runs();
			(t.func)(&mut suite);
			suite_results.push(suite.results());
		}
		let mut suites_failed = 0;
		let combined_suite_results =
			suite_results
				.iter()
				.fold(TestSuiteResult::default(), |mut acc, item| {
					acc.tests += item.tests;
					acc.failed += item.failed;
					acc.skipped += item.skipped;
					if item.failed > 0 {
						suites_failed = suites_failed + 1
					}
					acc
				});

		println!("");
		if combined_suite_results.tests == 0 {
			log!(String::from("No Tests Found\n").red());
			return Ok(());
		}

		if combined_suite_results.failed == 0 {
			log!("All tests passed\n".bold().cyan().underlined());
		}

		print_summary(
			"Test Suites:\t".to_string(),
			suite_results.len() as u32,
			suites_failed,
			0,
		);
		print_summary(
			"Tests:\t\t".to_string(),
			combined_suite_results.tests,
			combined_suite_results.failed,
			combined_suite_results.skipped,
		);
		print_time(start_time);

		if args.watch {
			return Ok(());
		}
		utility::Terminal::show_cursor();
		expect(suites_failed).to_be(0)?;
		Ok(())
	}
}

fn parse_args() -> Args {
	let mut args = utility::CliArgs::get();

	let watch = tern!(vec_contains_str("-w", &args);true;false);
	args.retain(|v| !arr_contains_str(v, FLAGS));
	Args { watch, files: args }
}



fn print_summary(prefix: String, total: u32, failed: u32, skipped: u32) {
	let passed = total - failed - skipped;
	let mut summaries: Vec<&str> = Vec::new();
	let passed_str = format!("{passed} passed").bold().green();
	let skipped_str = format!("{skipped} skipped").bold().yellow();
	let failed_str = format!("{failed} failed").bold().red();
	let total_str =
		tern!(passed == total;format!("{total} total");format!("{passed} of {total} total"));
	if failed > 0 {
		summaries.push(&failed_str);
	}
	if skipped > 0 {
		summaries.push(&skipped_str);
	}
	summaries.push(&passed_str);
	summaries.push(&total_str);

	log!(prefix.bold() summaries.join(", "));
}


fn print_time(start: Instant) {
	let millis = Instant::now().duration_since(start).as_millis();
	let prefix = "Time:\t\t".bold();
	if millis < 100 {
		println!("{}{} ms", prefix, millis);
	} else {
		println!("{}{:.2} s", prefix, millis as f32 * 0.001);
	}
}

/*
Test Suites: 3 skipped, 42 passed, 42 of 45 total
Tests:       9 skipped, 109 passed, 118 total
Snapshots:   1 passed, 1 total
Time:        23.497 s
Ran all test suites.
*/
