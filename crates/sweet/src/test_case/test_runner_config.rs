pub struct TestRunnerConfig {
	pub watch: bool,
	pub files: Vec<String>,
}

impl TestRunnerConfig {
	#[cfg(not(target_arch = "wasm32"))]
	pub fn from_cli_args() -> Self {
		let mut args = forky_fs::cli_args::get();
		let watch = vec_contains_str("-w", &args);
		args.retain(|v| !arr_contains_str(v, FLAGS));
		Self { watch, files: args }
	}

	pub fn suite_passes_filter(&self, path: &str) -> bool {
		let matchable_path = path.replace('\\', "/");
		self.files.len() == 0
			|| self.files.iter().any(|a| matchable_path.contains(a))
	}
}

fn vec_contains_str(path: &str, args: &Vec<String>) -> bool {
	args.iter().any(|a| a == path)
}
fn arr_contains_str(path: &str, arr: &[&str]) -> bool {
	arr.iter().any(|a| *a == path)
}

const FLAGS: &'static [&str] = &["-w"];
