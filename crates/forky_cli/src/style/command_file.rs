// use super::*;
use super::*;
use anyhow::Result;
use clap::Arg;
use clap::ArgMatches;
use forky_fs::terminal;
use forky_fs::Subcommand;
// use std::fs;

pub struct StyleCommandFile;

impl Subcommand for StyleCommandFile {
	fn name(&self) -> &'static str { "file" }
	fn about(&self) -> &'static str { "Apply to specified style file." }

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command
			.arg(Arg::new("in").required(true))
			.arg(Arg::new("out").required(true))
	}
	fn run(&self, args: &ArgMatches) -> Result<()> {
		terminal::print_forky();
		let path_in = args.get_one::<String>("in").unwrap();
		let path_out = args.get_one::<String>("out").unwrap();
		parse_to_file(&path_in, &path_out)?;
		Ok(())
	}
}
