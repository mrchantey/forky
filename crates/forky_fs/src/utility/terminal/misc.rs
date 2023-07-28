use crossterm::*;
use forky_core::*;
use std::fmt::Display;
use std::io::stdout;
use std::io::Write;


pub fn clear() { clear_terminal().unwrap(); }

pub fn print_forky() {
	println!("\n🤘 lets get forky! 🤘\n");
}

pub fn show_cursor() {
	let mut stdout = stdout();
	stdout.execute(cursor::Show).unwrap();
}


pub fn reset_cursor() {
	let mut stdout = stdout();
	stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

fn clear_terminal() -> Result<()> {
	let mut stdout = stdout();
	stdout
		.queue(terminal::Clear(terminal::ClearType::Purge))?
		.queue(cursor::Hide)?
		.queue(cursor::MoveTo(0, 0))?;
	stdout.flush()?;
	Ok(())
}
