use super::*;
use clap::Arg;
use clap::ArgAction;
use clap::Command;
use forky_fs::Subcommand;

pub struct SweetCommand;

const ABOUT: &str = "build the wasm sweet runner and start a dev server";

impl Subcommand for SweetCommand {
	fn name(&self) -> &'static str { "sweet" }
	fn about(&self) -> &'static str { ABOUT }
	fn append_command(&self, command: Command) -> Command {
		command
			.arg(
				Arg::new("specify-suite")
					.required(false)
					.action(ArgAction::Append),
			)
			.arg(
				Arg::new("package")
					.required(false)
					.short('p')
					.long("package"),
			)
			.arg(
				Arg::new("release")
					.required(false)
					.long("release")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("secure")
					.required(false)
					.long("secure")
					.action(ArgAction::SetTrue),
			)
	}
	fn run(&self, args: &clap::ArgMatches) -> anyhow::Result<()> {
		let mut cli = SweetCli::default();
		cli.package = args.get_one::<String>("package").cloned();
		cli.server.address.secure = args.get_flag("secure");
		cli.release = args.get_flag("release");
		cli.run()
	}
}
