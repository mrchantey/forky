use super::*;
use forky_core::prelude::*;
use forky_fs::prelude::*;

pub struct AutoModCommand;

fn watcher() -> FsWatcher {
	FsWatcher::default()
		.with_watch("**/*.rs")
		.with_ignore("{justfile,.gitignore,target,html}")
		//i think you can remove all except target, im debouncing already
		.with_ignore("**/*_g.rs")
		.with_ignore("**/mod.rs")
}


impl AutoModCommand {
	pub fn run_with_mutex(&self, mutex: ArcMut<()>) -> anyhow::Result<()> {
		let mut watcher = watcher();
		watcher.quiet = true;
		watcher.with_mutex(mutex).watch(|_| run::run())
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
