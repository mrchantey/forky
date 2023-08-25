// use crate::*;
use colorize::*;
// use std::default;

#[derive(Debug, Default)]
pub struct SuiteResult {
	pub file: String,
	pub tests: usize,
	pub skipped: usize,
	pub failed: Vec<String>,
}


impl SuiteResult {
	pub fn new(file: String, tests: usize, skipped: usize) -> Self {
		SuiteResult {
			file,
			tests,
			skipped,
			failed: Vec::new(),
		}
	}

	pub fn in_progress_str(&self) -> String {
		let mut value = " RUNS ".black().bold().yellowb();
		value += " ";
		value += pretty_path(&self.file).as_str();
		value
	}


	pub fn end_str(&self) -> String {
		let mut val = if self.failed.len() == 0 {
			" PASS ".black().bold().greenb()
		} else {
			" FAIL ".black().bold().redb()
		};
		val += " ";
		val += pretty_path(&self.file).as_str();

		val += &self
			.failed
			.iter()
			.fold(String::new(), |val, err| val + err.to_string().as_str());

		val
	}
}



fn pretty_path(file: &str) -> String {
	let mut splt = file.split("\\").collect::<Vec<&str>>();
	let _file = splt.pop();
	let file = _file.unwrap_or_default().to_string().bold();
	let path = splt.join("\\").faint();
	let middle = "\\".to_string().faint();
	format!("{path}{middle}{file}")
}
