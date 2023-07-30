use forky_core::OsStrOptionX;
use forky_core::StrX;
use std::path::PathBuf;

pub fn filename_starts_with_uppercase(p: &PathBuf) -> bool {
	p.file_name().str().first().is_ascii_uppercase()
}

pub fn filename_included(p: &PathBuf, arr: &[&str]) -> bool {
	arr.iter().any(|f| p.file_stem().unwrap() == *f)
}
pub fn filename_starts_with_underscore(p: &PathBuf) -> bool {
	p.file_name().str().first() == '_'
}
pub fn filename_ends_with_underscore(p: &PathBuf) -> bool {
	p.file_name().str().last() == '_'
}

pub fn filename_contains_double_underscore(p: &PathBuf) -> bool {
	p.file_name().str().contains("__")
}
