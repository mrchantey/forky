use std::{fs, path::Path, path::PathBuf};


pub fn read_dir_recursive(path: PathBuf) -> Vec<PathBuf> {
	let acc: Vec<PathBuf> = Vec::new();
	_read_dir_recursive(acc, path)
}
pub fn _read_dir_recursive(mut acc: Vec<PathBuf>, path: PathBuf) -> Vec<PathBuf> {
	if !path.is_dir() {
		return acc;
	}
	let children = fs::read_dir(&path).unwrap();
	acc.push(path);
	children
		.map(|c| c.unwrap().path())
		.fold(acc, _read_dir_recursive)
}



pub fn run_for_crate(path: PathBuf) {
	let child = path.join("src");
	let mut paths = read_dir_recursive(child)
		.into_iter()
		.filter(|p| p.file_stem().unwrap() != "src");
	// paths.for
	create_mod(&paths.next().unwrap());
	// for path in paths {
	// 	println!("Name: {}", path.display());
	// }
}

pub fn create_mod(path: &PathBuf) {
	let children = fs::read_dir(&path).unwrap();
	let mut str = String::from("#![allow(dead_code, unused_imports, unused_variables)]\n\n");
	children
		.map(|c| c.unwrap().path())
		.map(|c| c.file_stem().unwrap().to_owned())
		.map(|c| c.to_str().unwrap().to_owned())
		.for_each(|c| {
			str.push_str(&["mod ", &c[..], ";\npub use ", &c[..], "::*;\n"].join("")[..])
		});
	// println!("{}", str);
	let mut mod_path = path.clone();
	mod_path.push("mod.rs");
	// let mod_path = Path::from(&path.to_str());
	// path.push("mod.rs");
	fs::write(&mod_path, str).unwrap();
	println!("wrote to {}", &path.to_str().unwrap());
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
