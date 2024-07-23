use anyhow::Result;
use glob::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// read directory and return all sub directories
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

/// get all directories matching a glob pattern, removing duplicates
pub fn directories_matching(pattern: &str) -> Vec<PathBuf> {
	glob(pattern)
		.unwrap()
		.filter_map(|val| val.ok())
		.map(|val| {
			if val.is_dir() {
				val
			} else {
				val.parent().unwrap().to_path_buf()
			}
		})
		.collect::<HashSet<PathBuf>>()
		// .fold(HashSet::new(), |mut acc, val| {
		// 	acc.insert(val);
		// 	acc
		// })
		.iter()
		.map(|path| path.clone())
		.collect::<Vec<_>>()
}

pub fn copy_recursive(
	source: impl AsRef<Path>,
	destination: impl AsRef<Path>,
) -> Result<()> {
	fs::create_dir_all(&destination)?;
	for entry in fs::read_dir(source)? {
		let entry = entry?;
		let filetype = entry.file_type()?;
		if filetype.is_dir() {
			copy_recursive(
				entry.path(),
				destination.as_ref().join(entry.file_name()),
			)?;
		} else {
			fs::copy(
				entry.path(),
				destination.as_ref().join(entry.file_name()),
			)?;
		}
	}
	Ok(())
}

// pub fn dir_contains(path: PathBuf, pattern: &str) -> bool {
// 	let pattern = Pattern::new(pattern).unwrap();
// 	glob::glob_with(
// 		&pattern.to_string(),
// 		glob::MatchOptions {
// 			case_sensitive: false,
// 			require_literal_separator: false,
// 			require_literal_leading_dot: false,
// 		},
// 	)
// 	read_dir_recursive(path)
// 		.iter()
// 		.any(|p| pattern. p.to_str().unwrap().contains(pattern))
// }
