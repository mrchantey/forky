use super::*;
use anyhow::Result;
use clap::ArgMatches;
use forky_fs::terminal;
use forky_fs::Subcommand;
use forky_fs::WatchConfig;

pub struct StyleCommandAll;

impl Subcommand for StyleCommandAll {
	fn name(&self) -> &'static str { "all" }
	fn about(&self) -> &'static str { "Apply to all style directories." }

	fn run(&self, _args: &ArgMatches) -> Result<()> {
		forky_fs::watch_path(
			&WatchConfig::new()
				.with_watch("**/*.css")
				.with_ignore("**/*/index.css")
				.with_ignore("**/html/style.css"),
			|_| {
				terminal::clear();
				terminal::print_forky();
				create_type_files().unwrap();
				create_index_files().unwrap();
			},
		)?;
		Ok(())
	}
}
