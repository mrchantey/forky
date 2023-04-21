use super::Subcommand;
use anyhow::Result;
use clap::{ArgMatches, Command};

pub struct AstroCommand;

impl Subcommand for AstroCommand {
	fn name(&self) -> &'static str { "astro" }
	fn run(&self, _args: &ArgMatches) -> Result<()> {
		println!("lets astro!");
		Ok(())
	}
	fn command(&self, command: Command) -> Command {
		command.about("get an astrology reading")
	}
}
