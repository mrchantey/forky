use super::*;
use crate::*;
use anyhow::Result;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;
use forky_fs::*;

pub struct RunTestsNativeCommand;

impl Subcommand for RunTestsNativeCommand {
	fn name(&self) -> &'static str { "Sweet Test Runner" }
	fn about(&self) -> &'static str { "Native runner for your tests." }


	fn append_command(&self, command: Command) -> Command {
		command
			.arg(
				Arg::new("filter")
					.help("filter by directory or path name, ie. `/test1.rs /e2e/`")
					.required(false)
					.action(ArgAction::Append),
			)
			.arg(
				Arg::new("watch")
					.help("clears screen and does not return error")
					.required(false)
					.short('w')
					.long("watch")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("parallel")
					.help("run tests in parallel")
					.required(false)
					.short('p')
					.long("parallel")
					.action(ArgAction::SetTrue),
			)
	}

	fn run(&self, args: &ArgMatches) -> Result<()> {
		let config: TestRunnerConfig = args.into();
		TestRunnerNative::run(&config)
	}
}
