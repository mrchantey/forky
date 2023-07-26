use super::*;
use forky_fs::Subcommand;

pub struct AutoModCommand;

impl Subcommand for AutoModCommand {
	fn name(&self) -> &'static str { "mod" }
	fn about(&self) -> &'static str { "generate mod files for your project" }

	fn run(&self, _args: &clap::ArgMatches) -> anyhow::Result<()> {
		runner::run()
	}
}
