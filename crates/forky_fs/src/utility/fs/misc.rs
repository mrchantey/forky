use std::ffi::OsString;
use std::fs::read_dir;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::{env, io};

pub fn project_root() -> io::Result<PathBuf> {
	let path = env::current_dir()?;
	let mut path_ancestors = path.as_path().ancestors();

	while let Some(p) = path_ancestors.next() {
		let has_cargo = read_dir(p)?
			.into_iter()
			.any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
		if has_cargo {
			return Ok(PathBuf::from(p));
		}
	}
	Err(io::Error::new(
		ErrorKind::NotFound,
		"Ran out of places to find Cargo.toml",
	))
}
