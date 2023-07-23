use super::*;
use crate::*;


impl TestRunnerConfig {
	pub fn from_search_params() -> Self {
		// let params = forky_core::wasm::get_search_params();

		Self {
			watch: false,
			parallel: false,
			files: Vec::new(),
		}
	}
}
