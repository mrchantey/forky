use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io;

use crate::tarot::run_tarot_interpreter;

/// Welcome to the Mystic CLI!
#[derive(Debug, Parser)]
// #[command(about = "Welcome to the Mystic CLIer!")]
struct MysticCli {
	#[command(subcommand)]
	command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
	/// Create a tarot spread
	Tarot,
	Astrology,
}

pub async fn run() -> Result<()> {
	let result = match MysticCli::try_parse() {
		Ok(value) => value,
		Err(err) => {
			println!("{}", err);
			return Ok(());
			// return Err(err.into());
		}
	};

	let command = match result.command {
		Some(value) => value,
		None => return Ok(()),
	};

	match command {
		Command::Tarot => run_tarot_interpreter().await?,
		_ => todo!(),
	}
	// println!("{:?}", MysticCli::parse());
	// println!("{:?}", MysticCli::parse_from(["--help"]));
	Ok(())
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
