use crate::utils::CliPathExt;
use anyhow::Result;
use clap::Parser;
use forky_core::prelude::*;
use forky_fs::prelude::*;
use glob::Pattern;
use std::fs;
use std::path::Path;
use std::path::PathBuf;


/// These directories are treated especially
/// 1. They will not contain a mod file themselves
/// 2.
const IGNORE_FILES: &'static [&str] = &["mod"];

/// generate mod files for your project
#[derive(Debug, Default, Clone, Parser)]
#[command(name = "mod")]
pub struct AutoModCommand {
	/// Default points of entry, speeds up the process
	#[arg(
		value_parser = clap::value_parser!(PathBuf),
		default_value = "src,macros,cli,crates",
		value_delimiter = ',',
)]
	entry_dirs: Vec<PathBuf>,
	/// Paths containing rust files
	#[arg(
		long,
		value_parser = clap::value_parser!(PathBuf),
		default_value = "src,examples,test,tests",
		value_delimiter = ',',
)]
	rust_roots: Vec<PathBuf>,
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
	/// Log to stdout instead of file
	#[arg(short, long)]
	dry: bool,
	/// Minimal output
	#[arg(short, long)]
	quiet: bool,
	/// Log contents of files to stdout
	#[arg(short, long)]
	verbose: bool,
}

fn parse_glob(s: &str) -> Result<Pattern> {
	Ok(Pattern::new(&format!("*{s}*"))?)
}


fn any_match(patterns: &[Pattern], path: &PathBuf) -> bool {
	patterns.iter().any(|p| p.matches_path(path))
}

impl AutoModCommand {
	pub fn run(self) -> anyhow::Result<()> {
		Self::watcher().watch(|_| {
			println!("running auto mod");
			self.run_inner()
		})
	}

	pub fn run_with_mutex(&self, mutex: ArcMut<()>) -> anyhow::Result<()> {
		let mut watcher = Self::watcher();
		watcher.quiet = true;
		watcher.with_mutex(mutex).watch(|_| self.run_inner())
	}

	pub fn run_inner(&self) -> Result<()> {
		let mod_files = ReadDir {
			dirs: true,
			recursive: true,
			..Default::default()
		}
		.read_dirs_ok(&self.entry_dirs)?
		.into_iter()
		// dont create mod at entry dirs
		.filter(|p| !self.entry_dirs.iter().any(|d| d == p))
		// ignore anything matching exclude_glob, ie target
		.filter(|p| !any_match(&self.exclude_glob, p))
		// roots themselves should not have mod files
		.filter(|p| !CliPathExt::filename_included(p, &self.rust_roots))
		// only files in rust roots should have mod files
		.filter(|p| CliPathExt::anscestors_includes(p, &self.rust_roots))
		// ignore any dir that starts with an underscore
		.filter(|p| !CliPathExt::filestem_starts_with_underscore(p))
		.map(|p| {
			let text = self.create_mod_text(&p)?;
			self.save_to_file(&p, &text)?;
			Ok((p, text))
		})
		.collect::<FsResult<Vec<_>>>()?;
		self.print(mod_files);

		Ok(())
	}

	fn no_reexport(&self, path: &Path) -> bool {
		let path = PathExt::to_forward_slash_str(path);
		self.no_reexport
			.iter()
			.any(|pattern| pattern.matches(&path))
	}

	pub fn create_mod_text(&self, path: &Path) -> FsResult<String> {
		let mut filenames = ReadDir::all(&path)?
			.into_iter()
			.filter(|p| !CliPathExt::filename_included(p, IGNORE_FILES))
			.filter(|p| !CliPathExt::filestem_starts_with_underscore(p))
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
		Ok(files_str)
	}

	fn save_to_file(&self, path: &PathBuf, content: &str) -> FsResult<()> {
		if self.dry {
			return Ok(());
		}
		let file_name = if path.file_name().str() == "src" {
			"lib.rs"
		} else {
			"mod.rs"
		};
		let mut mod_path = path.clone();
		mod_path.push(file_name);
		fs::write(&mod_path, content).unwrap();
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
	fn print(&self, mut files: Vec<(PathBuf, String)>) {
		if self.quiet {
			return;
		}
		files.sort_by(|a, b| a.0.cmp(&b.0));

		for (path, content) in files {
			let prefix = if self.dry { "would create" } else { "creating" };
			println!("{} {}", prefix, path.to_str().unwrap(),);
			if self.verbose {
				println!("{}\n", content);
			}
		}
	}
}


#[cfg(test)]
mod test {
	use crate::common::AutoModCommand;
	use forky_fs::fs::FsExt;
	use std::path::*;
	use sweet::prelude::*;

	#[test]
	fn works() {
		const EXPECTED: &str = "pub mod included_dir;\npub mod included_file;\n#[allow(unused_imports)]\npub use self::included_file::*;\n";

		let txt = AutoModCommand::default()
			.create_mod_text(
				&FsExt::workspace_root()
					.join(Path::new("crates/forky_fs/test_dir")),
			)
			.unwrap();
		// let path = Path::new("crates/forky_cli/tests/test_dir");

		expect(txt.as_str()).to_be(EXPECTED);
	}
}
