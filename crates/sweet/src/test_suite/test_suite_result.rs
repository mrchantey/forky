use colorize::*;

#[derive(Debug, Default, Clone)]
pub struct TestSuiteResult {
	pub tests: usize,
	pub failed: usize,
	pub skipped: usize,
}


impl TestSuiteResult {
	pub fn new() -> Self {
		TestSuiteResult {
			tests: 0,
			failed: 0,
			skipped: 0,
		}
	}
	pub fn pretty_print(&self, prefix: &'static str) -> String {
		let TestSuiteResult {
			tests,
			failed,
			skipped,
		} = self;
		let passed = tests - failed - skipped;
		let mut summaries: Vec<&str> = Vec::new();
		let passed_str = format!("{passed} passed").bold().green();
		let skipped_str = format!("{skipped} skipped").bold().yellow();
		let failed_str = format!("{failed} failed").bold().red();
		let total_str = if passed == *tests {
			format!("{tests} total")
		} else {
			format!("{passed} of {tests} total")
		};
		if *failed > 0 {
			summaries.push(&failed_str);
		}
		if *skipped > 0 {
			summaries.push(&skipped_str);
		}
		summaries.push(&passed_str);
		summaries.push(&total_str);
		format!("{}{}", prefix.bold(), summaries.join(", "))
	}
}
