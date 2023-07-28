use super::*;
use anyhow::Result;
use clap::ArgMatches;
use forky_fs::FsWatcher;
use forky_fs::Subcommand;

pub struct StyleCommandAll;

impl Subcommand for StyleCommandAll {
	fn name(&self) -> &'static str { "all" }
	fn about(&self) -> &'static str { "Apply to all style directories." }

	fn run(&self, _args: &ArgMatches) -> Result<()> {
		FsWatcher::new()
			.with_watch("**/*.css")
			.with_ignore("**/*/index.css")
			.with_ignore("**/html/style.css")
			.watch(|_| {
				create_type_files()?;
				create_index_files()
			})
	}
}
