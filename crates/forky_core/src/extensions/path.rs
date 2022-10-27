use std::{
	path::Path,
	path::{self, PathBuf}, io, error::Error,
};

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

pub trait PathX {
	fn relative(&self) -> Result<&Path,Box<dyn Error>>;
}

impl PathX for Path{
	fn relative(&self) -> Result<&Path,Box<dyn Error>> {
		let cwd = std::env::current_dir()?;
		let p = self.strip_prefix(cwd)?;
		Ok(p)		
	}
}
