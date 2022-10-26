use crate::*;
use crate::testing::*;
use colorize::*;
use std::time::Instant;

pub fn run() -> Result<(), MatcherError> {
	utility::Terminal::clear();
	log!("\n🤘 lets get forky! 🤘\n");

	let start_time = Instant::now();

	let mut suite_results: Vec<TestSuiteResult> = Vec::new();
	
	println!(""); //sacrificial println
	for t in inventory::iter::<TestSuiteDesc> {
		let mut suite = TestSuite::new(t);
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

	//TODO allow force OK execution for less noise
	// expect(suites_failed).to_be(0)?;
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


fn print_time(start:Instant){
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
