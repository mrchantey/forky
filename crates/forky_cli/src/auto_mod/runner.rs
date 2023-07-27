use anyhow::Result;
use forky_core::*;
use forky_fs::fs::read_dir_recursive;
use forky_fs::*;
use std::env;
use std::fs;
use std::path::PathBuf;

const CRATE_FOLDERS: &'static [&str] = &["src", "examples", "tests", "test"];
const IGNORE_FOLDERS: &'static [&str] = &["src", "examples", "tests", "test"];
const IGNORE_FILES: &'static [&str] = &["mod", "lib", "main", "_lib"];

pub fn run() -> Result<()> {
	terminal::clear();
	terminal::print_forky();
	match fs::read_dir("crates") {
		Ok(dirs) => dirs
			.map(|e| e.unwrap().path())
			.for_each(|p| run_for_crate(p)),
		_ => run_for_crate(env::current_dir()?),
	}
	// .unwrap();
	terminal::show_cursor();
	Ok(())
}

pub fn run_for_crate(path: PathBuf) {
	CRATE_FOLDERS
		.iter()
		.map(|s| PathBuf::push_with(&path, s))
		.for_each(run_for_crate_folder)
}

pub fn run_for_crate_folder(path: PathBuf) {
	read_dir_recursive(path)
		.into_iter()
		.filter(|p| !filename_included(p, IGNORE_FOLDERS))
		.map(|p| (create_mod_text(&p), p))
		.for_each(|(c, p)| save_to_file(&p, c))
}


fn save_to_file(path: &PathBuf, content: String) {
	// let file_name = "mod.rs";
	let file_name = if path.file_name().str() == "src" {
		"lib.rs"
	} else {
		"mod.rs"
	};
	let mut mod_path = path.clone();
	mod_path.push(file_name);
	fs::write(&mod_path, content).unwrap();
	println!("created mod file: {}", &mod_path.to_str().unwrap());
}

const PREFIX: &str = "";

pub fn create_mod_text(path: &PathBuf) -> String {
	let children = fs::read_dir(&path).unwrap();
	let parent_is_double_underscore =
		filename_contains_double_underscore(&path);

	let mut str = String::new();
	children
		.map(|p| p.unwrap().path())
		.filter(|c| !filename_included(c, IGNORE_FILES))
		.filter(|c| is_dir_or_rustfile(c))
		.for_each(|c| {
			let stem = c.file_stem().unwrap();
			let name = stem.to_str().unwrap().to_owned();
			let mut is_mod = c.is_dir() || parent_is_double_underscore;
			if filename_starts_with_underscore(&c) {
				is_mod = !is_mod;
			}
			if is_mod {
				str.push_str(&format!("pub mod {name};\n"));
			} else {
				#[rustfmt::skip]
				str.push_str(&format!("mod {name};\npub use self::{name}::*;\n"));
			}
		});
	str
}



// fn filename_starts_with_uppercase(p: &PathBuf) -> bool {
// 	p.file_name().str().first().is_ascii_uppercase()
// }
fn filename_included(p: &PathBuf, arr: &[&str]) -> bool {
	arr.iter().any(|f| p.file_stem().unwrap() == *f)
}
fn filename_starts_with_underscore(p: &PathBuf) -> bool {
	p.file_name().str().first() == '_'
}
fn filename_contains_double_underscore(p: &PathBuf) -> bool {
	p.file_name().str().contains("__")
}
fn is_dir_or_rustfile(p: &PathBuf) -> bool {
	match p.extension() {
		Some(value) => value.to_str().unwrap() == "rs",
		None => p.is_dir(),
	}
}
