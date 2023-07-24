use crate::*;
use forky_core::wasm::SearchParams;

impl TestRunnerConfig {
	pub fn from_search_params() -> Self {
		let mut files = Vec::new();
		if let Some(file) = SearchParams::get("file") {
			files.push(file);
		}
		Self {
			watch: false,
			parallel: false,
			files,
		}
	}
}
