use glob::Pattern;

#[derive(Debug, Default, Clone)]
pub struct TestRunnerConfig {
	pub watch: bool,
	pub parallel: bool,
	pub files: Vec<Pattern>,
}

impl TestRunnerConfig {
	pub fn suite_passes_filter(&self, path: &str) -> bool {
		let matchable_path = path;
		self.files.len() == 0
			|| self.files.iter().any(|a| a.matches(matchable_path))
	}
}
