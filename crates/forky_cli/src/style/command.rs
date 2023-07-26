// use super::*;
use super::*;
use anyhow::Result;
use clap::Arg;
use clap::ArgMatches;
use forky_fs::Subcommand;
use std::fs;

pub struct StyleCommand;

impl Subcommand for StyleCommand {
	fn name(&self) -> &'static str { "style" }
	fn about(&self) -> &'static str { "Generate types for styles" }

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command
			.arg(Arg::new("in").required(true))
			.arg(Arg::new("out").required(true))
	}
	fn run(&self, args: &ArgMatches) -> Result<()> {
		let path_in = args.get_one::<String>("in").unwrap();
		let path_out = args.get_one::<String>("out").unwrap();
		let stylesheet =
			fs::read_to_string(&path_in).expect("Expected to read file");
		let classes = get_classes(&stylesheet);
		let out = classes_to_rust(classes);
		// println!("{:?}", out);
		fs::write(path_out, out)?;
		println!("wrote style types to {path_out}");
		Ok(())
	}
}
