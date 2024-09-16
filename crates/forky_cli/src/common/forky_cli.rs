use super::*;
use crate::*;
use forky_fs::prelude::*;

pub struct ForkyCli;

impl Subcommand for ForkyCli {
	fn name(&self) -> &'static str { "Forky CLI" }
	fn about(&self) -> &'static str { "Welcome to the Forky CLI!" }

	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> {
		vec![
			Box::new(AutoFs),
			Box::new(watch::WatchCommand),
			Box::new(server::ServerCommand),
			Box::new(style::StyleCommand),
			Box::new(auto_mod::AutoModCommand),
		]
	}
}
