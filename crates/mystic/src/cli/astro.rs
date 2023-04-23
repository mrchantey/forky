use super::Subcommand;

pub struct AstroCommand;

impl Subcommand for AstroCommand {
	fn name(&self) -> &'static str { "astro" }
	fn about(&self) -> &'static str { "get an astrology reading" }
}
