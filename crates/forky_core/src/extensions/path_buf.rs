use crate::prelude::*;
use extend::ext;
use std::path::Path;
use std::path::PathBuf;

#[ext]
pub impl PathBuf {
	fn push_with<P: AsRef<Path>>(parent: &PathBuf, path: P) -> Self {
		let mut child = parent.clone();
		child.push(path);
		child
	}
	fn filename_starts_with_uppercase(&self) -> bool {
		self.file_name().str().first().is_ascii_uppercase()
	}

	fn filename_included(&self, arr: &[&str]) -> bool {
		arr.iter().any(|f| self.file_stem().unwrap() == *f)
	}
	fn filestem_starts_with_underscore(&self) -> bool {
		self.file_stem().str().first() == '_'
	}
	fn filestem_ends_with_underscore(&self) -> bool {
		self.file_stem().str().last() == '_'
	}
	fn filestem_ends_with_double_underscore(&self) -> bool {
		self.file_stem().str().ends_with("__")
	}
	fn filestem_contains_double_underscore(&self) -> bool {
		self.file_stem().str().contains("__")
	}

	fn filestem_ends_with_triple_underscore(&self) -> bool {
		self.file_stem().str().ends_with("___")
	}

	fn parent_ends_with_underscore(&self) -> bool {
		match self.parent() {
			Some(parent) => {
				parent.to_path_buf().file_name().str().last() == '_'
			}
			None => false,
		}
	}
	fn parent_ends_with_double_underscore(&self) -> bool {
		match self.parent() {
			Some(parent) => {
				parent.to_path_buf().file_name().str().ends_with("__")
			}
			None => false,
		}
	}


	fn is_dir_or_extension(&self, ext: &str) -> bool {
		match self.extension() {
			Some(value) => value.to_str().unwrap() == ext,
			None => self.is_dir(),
		}
	}

	fn parents(&self) -> Vec<PathBuf> {
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

	fn pop_first_two_path_components(path: &str) -> PathBuf {
		let mut components = Path::new(path).components();
		components.next();
		components.next();
		components.as_path().to_path_buf()
	}
}
