use anyhow::Result;
use forky_core::prelude::*;
use forky_fs::prelude::*;
use forky_fs::utility::fs::read_dir_recursive;
use std::env;
use std::fs;
use std::path::PathBuf;

const CRATE_FOLDERS: &'static [&str] =
    &["src", "examples", "tests", "test", "macros/src", "cli/src"];
const IGNORE_FOLDERS: &'static [&str] = &["src", "examples", "test", "tests"];
const IGNORE_FILES: &'static [&str] = &["mod"];
const CRATE_DIRS: &'static [&str] = &["src", "crates", "crates/forky"];

pub fn run() -> Result<()> {
    let _ = CRATE_DIRS
        .into_iter()
        .map(|dir| {
            match fs::read_dir(dir) {
                Ok(dirs) => dirs
                    .map(|e| e.unwrap().path())
                    .for_each(|p| run_for_crate(p)),
                // what does this do?
                _ => run_for_crate(env::current_dir()?),
            }
            Ok(())
        })
        .collect::<Result<Vec<()>>>()?;
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
				.filter(|p| !p.filestem_starts_with_underscore())
        .map(|p| (create_mod_text(&p), p))
        .for_each(|(c, p)| save_to_file(&p, c))
}

pub fn create_mod_text(path: &PathBuf) -> String {

    let mut filenames = fs::read_dir(&path)
        .unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| !p.filename_included(IGNORE_FILES))
        .filter(|p| !p.filestem_starts_with_underscore())
        .filter(|p| p.is_dir_or_extension("rs"))
        .collect::<Vec<_>>();
    filenames.sort();

    let files_str: String = filenames
        .into_iter()
        .map(|p| {
            let stem = p.file_stem().unwrap();
            let name = stem.to_str().unwrap().to_owned();
            if p.is_dir() {
                format!("pub mod {name};\n")
            } else {
                // format!("mod {name};\npub use self::{name}::*;\n")
                format!("pub mod {name};\n#[allow(unused_imports)]\npub use self::{name}::*;\n")
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
