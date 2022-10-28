use forky_core::{utility::Terminal, *};
use std::{array, fs, path::Path, path::PathBuf};

pub fn run_auto_mod() {
	Terminal::clear();
	Terminal::get_forky();
	fs::read_dir("crates")
		.unwrap()
		.map(|e| e.unwrap().path())
		.for_each(|p| run_for_crate(p));
	Terminal::show_cursor();
}

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

fn filename_starts_with_underscore(p: &PathBuf) -> bool { p.file_name().str().first() == '_' }
fn filename_starts_with_uppercase(p: &PathBuf) -> bool {
	p.file_name().str().first().is_ascii_uppercase()
}

fn filename_included(p: &PathBuf, arr: &[&str]) -> bool {
	arr.iter().any(|f| p.file_stem().unwrap() == *f)
}


pub fn run_for_crate_folder(path: PathBuf) {
	read_dir_recursive(path)
		.into_iter()
		.filter(|p| !filename_included(p, CRATE_FOLDERS))
		.filter(|p| !filename_starts_with_underscore(p))
		.for_each(|p| create_mod(&p));
}

const CRATE_FOLDERS: &'static [&str] = &["src", "examples", "tests", "test"];
// const NO_MOD_FOLDERS:&'static [&str] = &["src", "examples"];
const IGNORE_FILES: &'static [&str] = &["mod", "lib", "main"];

pub fn run_for_crate(path: PathBuf) {
	CRATE_FOLDERS
		.iter()
		.map(|s| PathBuf::push_with(&path, s))
		.for_each(run_for_crate_folder)
}

const PREFIX: &str =
	"#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]\n\n";

pub fn create_mod(path: &PathBuf) {
	let children = fs::read_dir(&path).unwrap();
	let mut str = String::from(PREFIX);
	children
		.map(|p| p.unwrap().path())
		.filter(|p| !filename_starts_with_underscore(&p))
		.filter(|c| !filename_included(c, IGNORE_FILES))
		// .map(|c| c.file_stem().unwrap().to_owned())
		// .map(|c| c.to_str().unwrap().to_owned())
		// .filter(|c|c != "mod")
		// .for_each(|c| {
		// 	// if c.
		// 	str.push_str(&["mod ", &c[..], ";\npub use ", &c[..], "::*;\n"].join("")[..])
		// });
		.for_each(|c| {
			let stem = c.file_stem().unwrap();
			let name = stem.to_str().unwrap().to_owned();
			// if filename_starts_with_uppercase(&c) {
			// str.push_str(&["pub mod ", &name[..], ";\n"].join("")[..]);
			// }else{
			str.push_str(&["mod ", &name[..], ";\npub use ", &name[..], "::*;\n"].join("")[..]);
			// }
		});

	let mut mod_path = path.clone();
	mod_path.push("mod.rs");
	// let mod_path = Path::from(&path.to_str());
	// path.push("mod.rs");
	fs::write(&mod_path, str).unwrap();
	println!("created mod file: {}", &mod_path.to_str().unwrap());
	// println!("wrote to {}: \n {}", &path.to_str().unwrap(), str);
}
