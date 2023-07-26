use super::AstroCommand;
use super::TarotCommand;
use forky_fs::Subcommand;

pub struct MysticCommand;

impl Subcommand for MysticCommand {
	fn name(&self) -> &'static str { "Mystic CLI" }
	fn about(&self) -> &'static str { "Welcome to the Mystic CLI!" }

	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> {
		vec![Box::new(TarotCommand), Box::new(AstroCommand)]
	}
}
