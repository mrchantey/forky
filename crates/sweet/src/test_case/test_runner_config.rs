pub struct TestRunnerConfig {
	pub watch: bool,
	pub parallel: bool,
	pub files: Vec<String>,
}

impl TestRunnerConfig {
	pub fn suite_passes_filter(&self, path: &str) -> bool {
		let matchable_path = path.replace('\\', "/");
		self.files.len() == 0
			|| self.files.iter().any(|a| matchable_path.contains(a))
	}
}
