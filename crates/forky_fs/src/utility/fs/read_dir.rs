use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn read_dir_recursive(path: PathBuf) -> Vec<PathBuf> {
	_read_dir_recursive(Vec::new(), path)
}
fn _read_dir_recursive(mut acc: Vec<PathBuf>, path: PathBuf) -> Vec<PathBuf> {
	if !path.is_dir() {
		return acc;
	}
	let children = fs::read_dir(&path).unwrap();
	acc.push(path);
	children
		.map(|c| c.unwrap().path())
		.fold(acc, _read_dir_recursive)
}
