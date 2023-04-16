use anyhow::Result;
use clap::Parser;
use std::io;

#[derive(Debug, Parser)]
#[command(name = "run")]
struct MainInput {
	// #[command(subcommand)]
	// command: Option<Command>,
}

// enum Command {
// 	Save {},
// 	Help {},
// }

pub fn run() -> Result<()> {
	let intro = r#"
	Welcome to Mystic!
	Please enter your name:
	"#;
	println!("{intro}");
	hello_world()
}

pub fn hello_world() -> Result<()> {
	let mut input = String::new();
	let stdin = io::stdin(); // We get `Stdin` here.
	stdin.read_line(&mut input)?;

	let input = MainInput::parse_from(input.split_whitespace());

	println!("input {:?} ", input);

	Ok(())
}
