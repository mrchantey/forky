use crossterm::*;
use std::{io::{Write,stdout}, fmt::Display};


fn clear_terminal()->Result<()>{
	let mut stdout = stdout();
	stdout.queue(terminal::Clear(terminal::ClearType::All))?
	.queue(cursor::MoveTo(0,0))?;
	stdout.flush().unwrap();
	Ok(())
}

pub struct Terminal{}

impl Terminal{
	pub fn clear(){
		clear_terminal().unwrap();
	}

	// pub fn print<T:Display>(str: T){

	// }
	

}