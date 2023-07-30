use anyhow::Result;
use forky_core::*;
use forky_fs::fs::*;
use forky_fs::*;
use std::env;
use std::fs;
use std::path::PathBuf;

const CRATE_FOLDERS: &'static [&str] = &["src", "examples", "tests", "test"];
const IGNORE_FOLDERS: &'static [&str] = &["src", "examples"];
const IGNORE_FILES: &'static [&str] = &["mod", "lib", "main", "_lib", "sweet"];

pub fn run() -> Result<()> {
	match fs::read_dir("crates") {
		Ok(dirs) => dirs
			.map(|e| e.unwrap().path())
			.for_each(|p| run_for_crate(p)),
		_ => run_for_crate(env::current_dir()?),
	}
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

pub fn create_mod_text(path: &PathBuf) -> String {
	let parent_is_double_underscore =
		filename_contains_double_underscore(&path);

	fs::read_dir(&path)
		.unwrap()
		.map(|p| p.unwrap().path())
		.filter(|p| p.is_dir() || !filename_included(p, IGNORE_FILES))
		.filter(|p| is_dir_or_extension(p, "rs"))
		.filter(|p| !filename_ends_with_underscore(p))
		.map(|p| {
			let stem = p.file_stem().unwrap();
			let name = stem.to_str().unwrap().to_owned();
			let mut is_mod = p.is_dir() || parent_is_double_underscore;
			if filename_starts_with_underscore(&p) {
				is_mod = !is_mod;
			}
			if is_mod {
				format!("pub mod {name};\n")
			} else {
				format!("mod {name};\npub use self::{name}::*;\n")
			}
		})
		.collect()
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
