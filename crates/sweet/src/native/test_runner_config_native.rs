use crate::*;
use clap::ArgMatches;

impl TestRunnerConfig {
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
			// parallel: false,
			parallel: true,
			files: args,
		}
	}
}

impl From<&ArgMatches> for TestRunnerConfig {
	fn from(value: &ArgMatches) -> Self {
		let watch = value.get_flag("watch");
		let parallel = value.get_flag("parallel");
		let filters = value
			.get_many::<String>("filter")
			.unwrap_or_default()
			.map(|s| s.clone())
			.collect::<Vec<_>>();

		Self {
			watch,
			parallel,
			files: filters,
		}
	}
}
