// use super::*;
// use super::*;
use anyhow::Result;
// use clap::Arg;
use clap::ArgMatches;
use forky_fs::terminal;
use forky_fs::Subcommand;
// use std::fs;

pub struct StyleCommandAll;

impl Subcommand for StyleCommandAll {
	fn name(&self) -> &'static str { "all" }
	fn about(&self) -> &'static str { "Apply to all style directories." }

	fn run(&self, _args: &ArgMatches) -> Result<()> {
		terminal::print_forky();

		//TODO get all css files
		// create corresponding rust file
		// wrap in mod with same name as file

		Ok(())
	}
}
