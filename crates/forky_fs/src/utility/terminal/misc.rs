use crossterm::*;
use std::io::stdout;
use std::io::Write;
use anyhow::Result;

pub fn clear() { clear_terminal().unwrap(); }

pub fn print_forky() {
	println!("\n🦀 running forky! 🦀\n");
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
		.queue(terminal::Clear(terminal::ClearType::All))?
		// .queue(cursor::Hide)?
		.queue(cursor::MoveTo(0, 0))?;
	stdout.flush()?;
	Ok(())
}
