use std::{path::Path, path::PathBuf};

pub trait StringX {
	// fn to_str<P: AsRef<Path>>(parent: &PathBuf, path: P) -> Self;
	fn to_str(&self) -> &str;
}
impl StringX for String {
	fn to_str(&self) -> &str { &self[..] }
}
