use super::*;
use forky_core::ArcMut;
use forky_fs::FsWatcher;
use forky_fs::Subcommand;

pub struct AutoModCommand;

fn watcher() -> FsWatcher {
	FsWatcher::default()
		.with_watch("**/*.rs")
		.with_ignore("{justfile,.gitignore,target,html}")
		.with_ignore("**/*_g.rs")
		.with_ignore("**/mod.rs")
}


impl AutoModCommand {
	pub fn run_with_mutex(&self, mutex: ArcMut<()>) -> anyhow::Result<()> {
		watcher().with_mutex(mutex).watch(|_| run::run())
	}
}

impl Subcommand for AutoModCommand {
	fn name(&self) -> &'static str { "mod" }
	fn about(&self) -> &'static str { "generate mod files for your project" }

	fn run(&self, _args: &clap::ArgMatches) -> anyhow::Result<()> {
		watcher().watch(|_| run::run())
		// .watch_log()
	}
}
