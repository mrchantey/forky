use super::{AstroCommand, TarotCommand};

pub struct MysticCommand;

impl crate::cli::Subcommand for MysticCommand {
	fn name(&self) -> &'static str { "Mystic CLI" }
	fn about(&self) -> &'static str { "Welcome to the Mystic CLI!" }

	fn subcommands(&self) -> Vec<Box<dyn super::Subcommand>> {
		vec![Box::new(TarotCommand), Box::new(AstroCommand)]
	}
}
