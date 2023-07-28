use crate::*;
use forky_fs::Subcommand;

pub struct ForkyCli;

impl Subcommand for ForkyCli {
	fn name(&self) -> &'static str { "Forky CLI" }
	fn about(&self) -> &'static str { "Welcome to the Forky CLI!" }

	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> {
		vec![
			// Box::new(AutoFs),
			Box::new(style::StyleCommand),
			Box::new(auto_mod::AutoModCommand),
		]
	}
}
