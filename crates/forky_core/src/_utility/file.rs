use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn write<P>(filename: P, data: &str) -> std::io::Result<()>
where
	P: AsRef<Path>,
{
	let mut file = File::create(filename)?;
	file.write_all(data.as_bytes())?;
	Ok(())
}

// pub fn create_directory<P>(dir: P)
// where
// 	P: AsRef<Path>,
// {
// 	File::cret
// }
