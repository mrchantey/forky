use super::*;
use anyhow::Result;
use clap::Arg;
use forky_core::OptionTExt;
use forky_fs::Subcommand;

pub struct ServerCommand;

impl Subcommand for ServerCommand {
	fn name(&self) -> &'static str { "serve" }
	fn about(&self) -> &'static str { "serve static files" }

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command.arg(Arg::new("dir").required(false).default_value("html"))
	}

	fn run(&self, args: &clap::ArgMatches) -> Result<()> {
		let dir = args.get_one::<String>("dir").ok()?;
		Server::serve_forever(|| Server::default().with_dir(dir))
	}
}
