use super::StyleAllCli;
use anyhow::Result;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;
use forky_fs::prelude::*;

pub struct StyleCommandAll;

impl Subcommand for StyleCommandAll {
	fn name(&self) -> &'static str { "all" }
	fn about(&self) -> &'static str { "Apply to all style directories." }

	fn append_command(&self, command: Command) -> Command {
		command
			.arg(
				Arg::new("lightning")
					.help("specify directory for lightning to run")
					.required(false)
					.short('l')
					.long("lightning")
					.action(ArgAction::Set),
			)
			.arg(
				Arg::new("package")
					.help("package to find lib.css file in")
					.required(false)
					.short('p')
					.long("package")
					.action(ArgAction::Set),
			)
	}

	fn run(&self, args: &ArgMatches) -> Result<()> {
		let cli: StyleAllCli = args.into();
		cli.run()
	}
}
