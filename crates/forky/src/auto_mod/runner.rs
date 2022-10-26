use std::{fs, path::PathBuf};


pub fn read_dir_recursive(path: PathBuf) -> Vec<PathBuf> {
	let acc: Vec<PathBuf> = Vec::new();
	_read_dir_recursive(path, acc)
}
pub fn _read_dir_recursive(path: PathBuf, mut acc: Vec<PathBuf>) -> Vec<PathBuf> {
	let children = fs::read_dir(&path).unwrap();
	acc.push(path);
	for child in children {
		// if(child)
		let child_path = child.unwrap().path();
		if child_path.is_dir() {
			acc = _read_dir_recursive(child_path, acc);
		}
		// println!("Child! {}", child.unwrap().path().display());
	}
	acc
}

pub fn run_for_crate(path: PathBuf) {
	let child = path.join("src");
	let paths = read_dir_recursive(child);
	for path in paths {
		println!("Name: {}", path.display());
	}
}

pub fn run_auto_mod() {
	// let paths = fs::read_dir("./crates").unwrap().map(|val|{
	// 	val
	// });
	fs::read_dir("crates")
		.unwrap()
		.map(|e| e.unwrap().path())
		.for_each(|p| run_for_crate(p));
	// .filter_map(|p| p);
	// .filter(|p| p.is_some());
	// .map(|p| Some(p));
}
