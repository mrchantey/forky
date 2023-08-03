use super::*;
use anyhow::Result;
use clap::ArgMatches;
use forky_core::ArcMut;
use forky_fs::FsWatcher;
use forky_fs::Subcommand;

pub struct StyleCommandAll;


impl StyleCommandAll {
	pub fn run_with_mutex(&self, mutex: ArcMut<()>) -> anyhow::Result<()> {
		watcher().with_mutex(mutex).watch(|_| run())
	}
}

impl Subcommand for StyleCommandAll {
	fn name(&self) -> &'static str { "all" }
	fn about(&self) -> &'static str { "Apply to all style directories." }

	fn run(&self, _args: &ArgMatches) -> Result<()> {
		watcher().watch(|_| run())
	}
}

fn watcher() -> FsWatcher {
	FsWatcher::default()
		.with_watch("**/*.css")
		.with_ignore("**/index.css")
		.with_ignore("**/html/**")
		.with_ignore("**/target/**")
}

fn run() -> Result<()> {
	create_type_files()?;
	create_index_files()
}
