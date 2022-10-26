use std::{path::Path, path::PathBuf};

pub trait PathBufX {
	fn push_with<P: AsRef<Path>>(parent: &PathBuf, path: P) -> Self;
	// fn push_with(&self)->&Self;
}
impl PathBufX for PathBuf {
	fn push_with<P: AsRef<Path>>(parent: &PathBuf, path: P) -> Self {
		let mut child = parent.clone();
		child.push(path);
		child
	}
}
