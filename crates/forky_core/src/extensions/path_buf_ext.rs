use extend::ext;
use std::path::PathBuf;

#[ext]
pub impl PathBuf {
	fn is_dir_or_extension(&self, ext: &str) -> bool {
		match self.extension() {
			Some(value) => value.to_str().unwrap() == ext,
			None => self.is_dir(),
		}
	}


	/// List of all directories, including self if it is a directory.
	fn dir_parts(&self) -> Vec<PathBuf> {
		let mut acc = Vec::new();
		let mut current = self.clone();
		if self.is_dir() {
			acc.push(self.clone());
		}
		while let Some(parent) = current.parent() {
			acc.push(parent.to_path_buf());
			current = parent.to_path_buf();
		}
		acc
	}
}
