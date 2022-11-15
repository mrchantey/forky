use forky_core::*;
use forky_fs::*;
use std::{array, fs, path::Path, path::PathBuf};

pub fn run() {
	terminal::clear();
	terminal::get_forky();
	fs::read_dir("crates")
		.unwrap()
		.map(|e| e.unwrap().path())
		.for_each(|p| run_for_crate(p));
	terminal::show_cursor();
}

pub fn read_dir_recursive(path: PathBuf) -> Vec<PathBuf> {
	let acc: Vec<PathBuf> = Vec::new();
	_read_dir_recursive(acc, path)
}
pub fn _read_dir_recursive(
	mut acc: Vec<PathBuf>,
	path: PathBuf,
) -> Vec<PathBuf> {
	if !path.is_dir() {
		return acc;
	}
	let children = fs::read_dir(&path).unwrap();
	acc.push(path);
	children
		.map(|c| c.unwrap().path())
		.fold(acc, _read_dir_recursive)
}

fn filename_starts_with_underscore(p: &PathBuf) -> bool {
	p.file_name().str().first() == '_'
}
fn filename_contains_double_underscore(p: &PathBuf) -> bool {
	p.file_name().str().contains("__")
}
fn filename_starts_with_uppercase(p: &PathBuf) -> bool {
	p.file_name().str().first().is_ascii_uppercase()
}

fn filename_included(p: &PathBuf, arr: &[&str]) -> bool {
	arr.iter().any(|f| p.file_stem().unwrap() == *f)
}


pub fn run_for_crate_folder(path: PathBuf) {
	read_dir_recursive(path)
		.into_iter()
		.filter(|p| !filename_included(p, IGNORE_FOLDERS))
		.map(|p| (create_mod_text(&p), p))
		.for_each(|(c, p)| save_to_file(&p, c))
}

const CRATE_FOLDERS: &'static [&str] = &["src", "examples", "tests", "test"];
const IGNORE_FOLDERS: &'static [&str] = &["src", "examples", "tests", "test"];
const IGNORE_FILES: &'static [&str] = &["mod", "lib", "main", "_lib"];

pub fn run_for_crate(path: PathBuf) {
	CRATE_FOLDERS
		.iter()
		.map(|s| PathBuf::push_with(&path, s))
		.for_each(run_for_crate_folder)
}

fn save_to_file(path: &PathBuf, content: String) {
	// let file_name = "mod.rs";
	let file_name = tern!(path.file_name().str() == "src" ; "lib.rs"; "mod.rs");
	let mut mod_path = path.clone();
	mod_path.push(file_name);
	fs::write(&mod_path, content).unwrap();
	println!("created mod file: {}", &mod_path.to_str().unwrap());
}

const PREFIX: &str = "";
// "#![cfg_attr(debug_assertions, allow(dead_code, unused_imports,unused_mut, unused_variables,unused_parens))]\n\n";
// "#![allow(dead_code, unused_imports,unused_mut, unused_variables,unused_parens)]\n\n";

pub fn create_mod_text(path: &PathBuf) -> String {
	let children = fs::read_dir(&path).unwrap();
	// let dir_is_double_underscore = filename_contains_double_underscore(&path);

	let mut str = String::from(PREFIX);
	children
		.map(|p| p.unwrap().path())
		.filter(|c| !filename_included(c, IGNORE_FILES))
		.for_each(|c| {
			let stem = c.file_stem().unwrap();
			let name = stem.to_str().unwrap().to_owned();
			let mut is_mod = c.is_dir();
			if filename_starts_with_underscore(&c) {
				is_mod = !is_mod;
			}
			if is_mod {
				str.push_str(&["pub mod ", &name[..], ";\n"].join("")[..]);
			} else {
				#[rustfmt::skip]
				str.push_str(&["mod ", &name[..], ";\npub use self::", &name[..], "::*;\n"].join("")[..]);
			}
		});
	str
}
