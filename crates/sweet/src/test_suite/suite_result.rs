use colorize::*;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
// use std::default;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SuiteResult {
	pub file: PathBuf,
	pub tests: usize,
	pub skipped: usize,
	pub failed: Vec<String>,
}


impl SuiteResult {
	pub fn new(file: PathBuf, tests: usize, skipped: usize) -> Self {
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
		value += Self::pretty_path(&self.file).as_str();
		value
	}


	pub fn end_str(&self) -> String {
		let mut val = if self.failed.len() == 0 {
			" PASS ".black().bold().greenb()
		} else {
			" FAIL ".black().bold().redb()
		};
		val += " ";
		val += Self::pretty_path(&self.file).as_str();

		val += &self
			.failed
			.iter()
			.fold(String::new(), |val, err| val + err.to_string().as_str());

		val
	}

	pub fn pretty_path(file: &PathBuf) -> String {
		let name = file
			.file_name()
			.unwrap_or_default()
			.to_string_lossy()
			.to_string()
			.bold();
		let dir = file
			.parent()
			.unwrap_or_else(|| Path::new(""))
			.to_string_lossy()
			.to_string()
			.faint();
		let slash = "/".faint();
		format!("{dir}{slash}{name}")
	}
}
