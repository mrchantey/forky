use crate::tarot::spread::{self, TarotSpread};

use super::Subcommand;
use anyhow::Result;
use clap::ArgMatches;
pub struct TarotCommand;

impl Subcommand for TarotCommand {
	fn name(&self) -> &'static str { "tarot" }
	fn about(&self) -> &'static str {
		"get a tarot reading, defaults to a three card spread."
	}
	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> {
		vec![Box::new(ThreeCard)]
	}
	fn run(&self, _args: &ArgMatches) -> Result<()> { ThreeCard.run(_args) }
}

pub struct ThreeCard;

impl Subcommand for ThreeCard {
	fn name(&self) -> &'static str { "three-card" }
	fn about(&self) -> &'static str { "Create a three card spread" }
	fn run(&self, _args: &ArgMatches) -> Result<()> {
		println!("{}", spread::PastPresentFuture::new());
		Ok(())
	}
}
