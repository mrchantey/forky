use super::Subcommand;
use crate::tarot::spread::{self, TarotSpread};
use anyhow::Result;
use clap::{ArgMatches, Command};



pub struct TarotCommand;

impl Subcommand for TarotCommand {
	fn name(&self) -> &'static str { "tarot" }
	fn run(&self, _args: &ArgMatches) -> Result<()> {
		println!("{}", spread::PastPresentFuture::new());
		Ok(())
	}
	fn command(&self, command: Command) -> Command {
		command.about("get a tarot reading.")
	}
}
