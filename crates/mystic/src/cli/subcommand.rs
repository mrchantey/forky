use anyhow::Result;
use clap::{ArgMatches, Command};

pub trait Subcommand {
	fn name(&self) -> &'static str;
	fn run(&self, args: &ArgMatches) -> Result<()>;
	fn get_command(&self) -> Command { self.command(Command::new(self.name())) }
	fn command(&self, command: Command) -> Command;
}
