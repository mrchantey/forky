use crate::tarot::spread;
use crate::tarot::spread::TarotSpread;
use crate::tarot::TarotDeck;
use anyhow::Result;
use clap::ArgMatches;
use forky_fs::Subcommand;

pub struct TarotCommand;

impl Subcommand for TarotCommand {
	fn name(&self) -> &'static str { "tarot" }
	fn about(&self) -> &'static str {
		"Get a tarot reading, defaults to a single card."
	}
	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> {
		vec![Box::new(OneCard), Box::new(ThreeCard)]
	}
	fn run(&self, _args: &ArgMatches) -> Result<()> { OneCard.run(_args) }
}

struct ThreeCard;

impl Subcommand for ThreeCard {
	fn name(&self) -> &'static str { "three" }
	fn about(&self) -> &'static str { "Create a three card spread." }
	fn run(&self, _args: &ArgMatches) -> Result<()> {
		println!("{}", spread::PastPresentFuture::new());
		Ok(())
	}
}
struct OneCard;

impl Subcommand for OneCard {
	fn name(&self) -> &'static str { "one" }
	fn about(&self) -> &'static str { "Draw a single card." }
	fn run(&self, _args: &ArgMatches) -> Result<()> {
		println!("{}", TarotDeck::shuffled().pop().unwrap());
		Ok(())
	}
}
