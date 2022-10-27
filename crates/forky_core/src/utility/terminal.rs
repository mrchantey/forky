use crate::log;
use crossterm::*;
use std::{
	fmt::Display,
	io::{stdout, Write},
};


fn clear_terminal() -> Result<()> {
	let mut stdout = stdout();
	stdout
		.queue(terminal::Clear(terminal::ClearType::All))?
		.queue(cursor::Hide)?
		.queue(cursor::MoveTo(0, 0))?;
	stdout.flush().unwrap();
	Ok(())
}

pub struct Terminal {}

impl Terminal {
	pub fn clear() { clear_terminal().unwrap(); }

	pub fn get_forky() {
		log!("\n🤘 lets get forky! 🤘\n");
	}

	pub fn show_cursor(){
		let mut stdout = stdout();
		stdout.execute(cursor::Show).unwrap();
	}

	// pub fn print<T:Display>(str: T){

	// }
}
