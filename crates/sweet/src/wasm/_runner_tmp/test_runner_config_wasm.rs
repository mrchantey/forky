use crate::*;
use forky_web::SearchParams;
use glob::Pattern;


impl TestRunnerConfig {
	pub fn from_search_params() -> Self {
		let mut matches = Vec::new();
		if let Some(file) = SearchParams::get("file") {
			//todo error onn malformed pattern
			matches.push(Pattern::new(&file).unwrap());
		}
		Self {
			watch: false,
			parallel: false,
			matches,
		}
	}
}
