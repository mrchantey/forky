use super::*;
use forky_fs::Subcommand;

pub struct StyleCommand;

impl Subcommand for StyleCommand {
	fn name(&self) -> &'static str { "style" }
	fn about(&self) -> &'static str { "Generate types for styles" }

	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> {
		vec![Box::new(StyleCommandAll), Box::new(StyleCommandFile)]
	}
}
