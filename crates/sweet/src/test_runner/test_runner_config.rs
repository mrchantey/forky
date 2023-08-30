use std::path::PathBuf;

use glob::Pattern;

#[derive(Debug, Clone)]
pub struct TestRunnerConfig {
	pub watch: bool,
	pub parallel: bool,
	pub silent: bool,
	pub matches: Vec<Pattern>,
}

impl Default for TestRunnerConfig {
	fn default() -> Self {
		Self {
			watch: false,
			parallel: false,
			silent: false,
			matches: Vec::new(),
		}
	}
}

impl TestRunnerConfig {
	pub fn suite_passes_filter(&self, path: &PathBuf) -> bool {
		let matchable_path = path.to_string_lossy();
		self.matches.len() == 0
			|| self.matches.iter().any(|a| a.matches(&matchable_path))
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
		if self.silent {
			out += format!("silent: true\n").as_str();
		}
		write!(f, "{}", out)
	}
}
