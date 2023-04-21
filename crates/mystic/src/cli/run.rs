// use super::{Subcommand, *};
use anyhow::Result;
use clap::{Command, Parser, Subcommand};
use std::{io, process};

use super::{AstroCommand, TarotCommand};

/// Welcome to the Mystic CLI!
#[derive(Debug, Parser)]
// #[command(about = "Welcome to the Mystic CLIer!")]
struct MysticCli {
	#[command(subcommand)]
	command: Option<MysticCommand>,
}

#[derive(Debug, Subcommand)]
enum MysticCommand {
	/// Create a tarot spread
	Tarot,
	Astrology,
}

pub fn run() -> Result<()> {
	let mut app = Command::new("Mystic CLI")
		.about("Welcome to the Mystic CLI!")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.allow_external_subcommands(true)
		.version("0.1.0");

	let subcommands: Vec<Box<dyn super::Subcommand>> =
		vec![Box::new(TarotCommand), Box::new(AstroCommand)];
	for sub in subcommands.iter() {
		app = app.subcommand(sub.get_command());
	}
	let mut success = false;
	for (name, args) in app.clone().get_matches().subcommand().iter() {
		for sub in subcommands.iter() {
			if sub.name() == *name {
				success = true;
				sub.run(args)?;
			}
		}
	}
	if !success {
		app.clone().print_help()?;
	}
	process::exit(0);
}

pub fn run_loop() -> Result<()> {
	let stdin = io::stdin(); // We get `Stdin` here.
	loop {
		let mut input = String::new();
		stdin.read_line(&mut input)?;

		let input = match MysticCli::try_parse_from(input.split_whitespace()) {
			Ok(value) => value,
			Err(err) => {
				println!("whoops! {}", err);
				continue;
			}
		};
		println!("{:?} ", input);
	}
}
