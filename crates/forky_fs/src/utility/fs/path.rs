use forky_core::OsStrOptionX;
use forky_core::StrX;
use std::path::Path;
use std::path::PathBuf;

pub fn filename_starts_with_uppercase(p: &PathBuf) -> bool {
	p.file_name().str().first().is_ascii_uppercase()
}

pub fn filename_included(p: &PathBuf, arr: &[&str]) -> bool {
	arr.iter().any(|f| p.file_stem().unwrap() == *f)
}
pub fn filestem_starts_with_underscore(p: &PathBuf) -> bool {
	p.file_stem().str().first() == '_'
}
pub fn filestem_ends_with_underscore(p: &PathBuf) -> bool {
	p.file_stem().str().last() == '_'
}
pub fn filestem_ends_with_double_underscore(p: &PathBuf) -> bool {
	p.file_stem().str().ends_with("__")
}
pub fn filestem_contains_double_underscore(p: &PathBuf) -> bool {
	p.file_stem().str().contains("__")
}

pub fn filestem_ends_with_triple_underscore(p: &PathBuf) -> bool {
	p.file_stem().str().ends_with("___")
}

pub fn parent_ends_with_underscore(p: &PathBuf) -> bool {
	match p.parent() {
		Some(parent) => parent.to_path_buf().file_name().str().last() == '_',
		None => false,
	}
}
pub fn parent_ends_with_double_underscore(p: &PathBuf) -> bool {
	match p.parent() {
		Some(parent) => parent.to_path_buf().file_name().str().ends_with("__"),
		None => false,
	}
}
pub fn pop_first_two_path_components(path: &str) -> PathBuf {
	let mut components = Path::new(path).components();
	components.next();
	components.next();
	components.as_path().to_path_buf()
}
