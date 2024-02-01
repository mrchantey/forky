use anyhow::Result;
use forky_core::*;
use forky_fs::fs::*;
use forky_fs::*;
use std::env;
use std::fs;
use std::path::PathBuf;

const CRATE_FOLDERS: &'static [&str] =
	&["src", "examples", "tests", "test", "macros/src", "cli/src"];
const IGNORE_FOLDERS: &'static [&str] = &["src", "examples"];
const IGNORE_FILES: &'static [&str] = &["mod", "lib", "main", "_lib", "sweet"];

pub fn run() -> Result<()> {
	match fs::read_dir("crates") {
		Ok(dirs) => dirs
			.map(|e| e.unwrap().path())
			.for_each(|p| run_for_crate(p)),
		_ => run_for_crate(env::current_dir()?),
	}
	match fs::read_dir("crates/forky") {
		Ok(dirs) => dirs
			.map(|e| e.unwrap().path())
			.for_each(|p| run_for_crate(p)),
		_ => {}
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
		.filter(|p| !p.filename_included(IGNORE_FOLDERS))
		.filter(|p| !p.filestem_ends_with_triple_underscore())
		.map(|p| (create_mod_text(&p), p))
		.for_each(|(c, p)| save_to_file(&p, c))
}

pub fn create_mod_text(path: &PathBuf) -> String {
	let treat_as_mod = path.filestem_contains_double_underscore();

	let files_str: String = fs::read_dir(&path)
		.unwrap()
		.map(|p| p.unwrap().path())
		.filter(|p| p.is_dir() || !p.filename_included(IGNORE_FILES))
		.filter(|p| !p.filestem_ends_with_triple_underscore())
		.filter(|p| p.is_dir_or_extension("rs"))
		.map(|p| {
			let stem = p.file_stem().unwrap();
			let name = stem.to_str().unwrap().to_owned();
			let mut is_mod = p.is_dir() || treat_as_mod;
			if p.filestem_starts_with_underscore() {
				is_mod = !is_mod;
			}
			if is_mod {
				format!("pub mod {name};\n")
			} else {
				// format!("mod {name};\npub use self::{name}::*;\n")
				format!("pub mod {name};\npub use self::{name}::*;\n")
			}
		})
		.collect();
	// format!("#![allow(unused_imports)]\n{files_str}")
	files_str
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
