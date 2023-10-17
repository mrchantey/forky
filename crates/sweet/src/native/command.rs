use super::*;
use crate::test_runner::*;
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
				Arg::new("match")
					.help("filter suites by path glob, ie `my_test` or `/e2e/`")
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
			.arg(
				Arg::new("silent")
					.help("don't log results")
					.required(false)
					.short('s')
					.long("silent")
					.action(ArgAction::SetTrue),
			)
	}

	fn run(&self, args: &ArgMatches) -> Result<()> {
		let mut config = TestRunnerConfig::from_arg_matchers(args)?;
		TestRunnerNative::run(&mut config)
	}
}
