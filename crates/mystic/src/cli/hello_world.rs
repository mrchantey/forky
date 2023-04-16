use anyhow::Result;
use std::io;

fn hello_world() -> Result<()> {
	let mut user_input = String::new();
	let stdin = io::stdin(); // We get `Stdin` here.
	stdin.read_line(&mut user_input);

	println!("input {} ", user_input);

	Ok(())
}
