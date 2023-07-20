pub struct TestRunnerConfig {
	pub watch: bool,
	pub parallel: bool,
	pub files: Vec<String>,
}

impl TestRunnerConfig {
	#[cfg(not(target_arch = "wasm32"))]
	pub fn from_cli_args() -> Self {
		fn vec_contains_str(path: &str, args: &Vec<String>) -> bool {
			args.iter().any(|a| a == path)
		}
		fn arr_contains_str(path: &str, arr: &[&str]) -> bool {
			arr.iter().any(|a| *a == path)
		}

		const FLAGS: &'static [&str] = &["-w"];
		let mut args = forky_fs::cli_args::get();
		let watch = vec_contains_str("-w", &args);
		args.retain(|v| !arr_contains_str(v, FLAGS));
		Self {
			watch,
			parallel: true,
			files: args,
		}
	}

	#[cfg(target_arch = "wasm32")]
	pub fn from_search_params() -> Self {
		// let params = forky_core::wasm::get_search_params();

		Self {
			watch: false,
			parallel: false,
			files: Vec::new(),
		}
	}


	pub fn suite_passes_filter(&self, path: &str) -> bool {
		let matchable_path = path.replace('\\', "/");
		self.files.len() == 0
			|| self.files.iter().any(|a| matchable_path.contains(a))
	}
}
