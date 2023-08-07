use glob::Pattern;

#[derive(Debug, Default, Clone)]
pub struct TestRunnerConfig {
	pub watch: bool,
	pub parallel: bool,
	pub matches: Vec<Pattern>,
}

impl TestRunnerConfig {
	pub fn suite_passes_filter(&self, path: &str) -> bool {
		let matchable_path = path;
		self.matches.len() == 0
			|| self.matches.iter().any(|a| a.matches(matchable_path))
	}
}

impl std::fmt::Display for TestRunnerConfig {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut out = String::new();
		let matches = self
			.matches
			.iter()
			.map(|m| m.to_string())
			.collect::<Vec<_>>()
			.join(" ");
		if self.watch {
			out += format!("watch: true\n").as_str();
		}
		if self.parallel {
			out += format!("parallel: true\n").as_str();
		}
		if self.matches.len() > 0 {
			out += format!("matching: {matches}\n").as_str();
		}
		write!(f, "{}", out)
	}
}
