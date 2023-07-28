use super::*;
use forky_fs::FsWatcher;
use forky_fs::Subcommand;

pub struct AutoModCommand;

impl Subcommand for AutoModCommand {
	fn name(&self) -> &'static str { "mod" }
	fn about(&self) -> &'static str { "generate mod files for your project" }

	fn run(&self, _args: &clap::ArgMatches) -> anyhow::Result<()> {
		FsWatcher::default()
			.with_watch("**/*.rs")
			.with_ignore("{justfile,.gitignore,target,html}")
			.with_ignore("**/*_g.rs")
			.with_ignore("**/mod.rs")
			.watch(|_| runner::run())
		// .watch_log()
	}
}
