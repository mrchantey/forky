use crate::utils::CliPathBufExt;
use anyhow::Result;
use clap::Parser;
use forky_core::prelude::*;
use forky_fs::prelude::*;
use glob::Pattern;
use std::fs;
use std::path::PathBuf;


/// These directories should not contain a mod file
const IGNORE_ROOTS: &'static [&str] = &["src", "examples", "test", "tests"];
const IGNORE_FILES: &'static [&str] = &["mod"];

/// generate mod files for your project
#[derive(Debug, Clone, Parser)]
#[command(name = "mod")]
pub struct AutoModCommand {
	/// Default points of entry
	#[arg(
		value_parser = clap::value_parser!(PathBuf),
		default_value = "src,macros,cli,crates",
		value_delimiter = ',',
)]
	entry_dirs: Vec<PathBuf>,
	/// Glob patterns where any match will still create a mod file but not reexport contents
	#[arg(long,value_parser=parse_glob)]
	no_reexport: Vec<Pattern>,
	#[arg(
		long,
		value_parser=parse_glob,
		default_value="*/target/**/*",
		value_delimiter=','
		)]
	exclude_glob: Vec<Pattern>,
}

fn parse_glob(s: &str) -> Result<Pattern> {
	Ok(Pattern::new(&format!("*{s}*"))?)
}


fn any_match(patterns: &[Pattern], path: &PathBuf) -> bool {
	patterns.iter().any(|p| p.matches_path(path))
}

impl AutoModCommand {
	pub fn run(self) -> anyhow::Result<()> {
		println!("running auto mod\n{:#?}", self);
		Self::watcher().watch(|_| self.run_inner())
	}

	pub fn run_with_mutex(&self, mutex: ArcMut<()>) -> anyhow::Result<()> {
		let mut watcher = Self::watcher();
		watcher.quiet = true;
		watcher.with_mutex(mutex).watch(|_| self.run_inner())
	}

	pub fn run_inner(&self) -> Result<()> {
		FsExt::read_dir_recursive_some(&self.entry_dirs)?
			.into_iter()
			.filter(|p| !any_match(&self.exclude_glob, p))
			.filter(|p| !p.filename_included(IGNORE_ROOTS))
			.filter(|p| !p.filestem_starts_with_underscore())
			.map(|p| {
				let text = self.create_mod_text(&p);
				self.save_to_file(&p, text)
			})
			.collect::<FsResult<Vec<_>>>()?;
		Ok(())
	}

	fn no_reexport(&self, path: &PathBuf) -> bool {
		let path = path.to_forward_slash_str();
		self.no_reexport
			.iter()
			.any(|pattern| pattern.matches(&path))
	}

	pub fn create_mod_text(&self, path: &PathBuf) -> FsResult<String> {
		let mut filenames = FsExt::read_dir(&path)?
			.into_iter()
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
							if self.no_reexport(&p) || p.is_dir() {
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

	fn save_to_file(&self, path: &PathBuf, content: String) -> FsResult<()> {
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
		Ok(())
	}
	fn watcher() -> FsWatcher {
		FsWatcher::default()
			.with_watch("**/*.rs")
			.with_ignore("{justfile,.gitignore,target,html}")
			//i think you can remove all except target, im debouncing already
			.with_ignore("**/*_g.rs")
			.with_ignore("**/mod.rs")
	}
}
