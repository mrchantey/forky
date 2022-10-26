use crate::testing::*;
use crate::utility::CliArgs;
use crate::*;
use colorize::*;
use std::path::PathBuf;
use std::slice::Iter;
use std::time::Instant;


fn suite_in_args(path: &str, args: &Vec<String>) -> bool { args.iter().any(|a| path.contains(a)) }
fn vec_contains_str(path: &str, args: &Vec<String>) -> bool { args.iter().any(|a| a == path) }
fn arr_contains_str(path: &str, arr: &[&str]) -> bool { arr.iter().any(|a| *a == path) }

const FLAGS: &'static [&str] = &["--ok"];

pub fn run() -> Result<(), MatcherError> {
	utility::Terminal::clear();
	log!("\n🤘 lets get forky! 🤘\n");

	let start_time = Instant::now();

	let mut suite_results: Vec<TestSuiteResult> = Vec::new();

	println!(""); //sacrificial println

	let mut args = CliArgs::get();
	let mut flags = args.clone();
	flags.retain(|v| arr_contains_str(v, FLAGS));
	args.retain(|v| !arr_contains_str(v, FLAGS));
	// let a = args.iter();
	// if(a.)

	for t in inventory::iter::<TestSuiteDesc> {
		if args.len() > 0 && !suite_in_args(t.file, &args) {
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
				if item.failed > 0 {
					suites_failed = suites_failed + 1
				}
				acc
			});

	println!("");
	if combined_suite_results.failed == 0 {
		log!("All tests passed".cyan().underlined());
	}

	print_summary(
		"Test Suites:\t".to_string(),
		suite_results.len() as u32,
		suites_failed,
		None,
	);
	print_summary(
		"Tests:\t\t".to_string(),
		combined_suite_results.tests,
		combined_suite_results.failed,
		None,
	);
	print_time(start_time);

	if vec_contains_str("--ok", &flags) {
		return Ok(());
	}
	expect(suites_failed).to_be(0)?;
	Ok(())
}

fn print_summary(prefix: String, total: u32, failed: u32, skipped: Option<u32>) {
	let skipped = skipped.unwrap_or(0);
	let passed = total - failed - skipped;
	let passed_str = format!("{passed} passed").bold().green();
	let summary = if failed > 0 {
		let failed_str = format!("{failed} failed").bold().red();
		let total_str = format!("{passed} of {total} total");
		[failed_str, passed_str, total_str].join(", ")
	} else {
		let total_str = format!("{total} total");
		[passed_str, total_str].join(", ")
	};
	log!(prefix.bold() summary);
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
