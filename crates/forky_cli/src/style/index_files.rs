use crate::utils::CliPathExt;
use anyhow::Result;
use forky_core::prelude::*;
use forky_fs::fs::ReadDir;
use glob::*;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn create_index_files() -> Result<()> {
	println!("\nstyle: creating index files...\n");
	remove_all_index_files()?;
	for_all_crates()?;
	Ok(())
}

fn remove_all_index_files() -> Result<()> {
	glob("**/index.css")
		.unwrap()
		.map(|path| fs::remove_file(path.unwrap()))
		.collect::<std::io::Result<()>>()?;
	Ok(())
}

fn for_all_crates() -> Result<()> {
	let pattern = Pattern::new("**/src/**/*.css").unwrap();
	let dirs_with_css = ReadDir::files_recursive("./")?
		.into_iter()
		.filter(|p| pattern.matches(&p.to_string_lossy()))
		.flat_map(|p| p.dir_parts())
		// .filter(|p| !p.filestem_ends_with_underscore())
		.collect::<HashSet<PathBuf>>();
	// dirs_with_css
	// 	.iter()
	// 	.for_each(|val| println!("css dir: {:?}", val));
	match ReadDir::dirs("crates") {
		Ok(dirs) => dirs
			.into_iter()
			.map(|p| for_crate(p, &dirs_with_css))
			.collect::<Result<()>>()?,
		_ => for_crate(env::current_dir()?, &dirs_with_css)?,
	}
	Ok(())
}

fn for_crate(path: PathBuf, dirs_with_css: &HashSet<PathBuf>) -> Result<()> {
	let path = CliPathExt::push_with(&path, "src");
	ReadDir::dirs_recursive(path)?
		.into_iter()
		.filter(|p| dirs_with_css.contains(p))
		//TODO filter by directories that contain any css
		.map(|p| (create_index_text(&p, dirs_with_css).unwrap(), p))
		.map(|(content, path)| save_to_disk(&path, content))
		.collect()
}

pub fn create_index_text(
	path: &PathBuf,
	dirs_with_css: &HashSet<PathBuf>,
) -> Result<String> {
	let ignore_files = Pattern::new("**/*/index.css").unwrap();

	let out = ReadDir::dirs(&path)?
		.into_iter()
		.filter(|p| !ignore_files.matches(p.to_str().unwrap()))
		.filter(|p| p.is_dir_or_extension("css"))
		.filter(|p| p.is_file() || dirs_with_css.contains(p))
		.map(|p| {
			let stem = p.file_stem().unwrap();
			let name = stem.to_str().unwrap().to_owned();
			if p.is_dir() {
				format!("@import './{name}/index.css';\n")
			} else {
				format!("@import './{name}.css';\n")
			}
		})
		.collect();
	Ok(out)
}


fn save_to_disk(path: &PathBuf, content: String) -> Result<()> {
	let mut path = path.clone();
	path.push("index.css");
	println!("created: {:?}", path);
	// println!("path: {:?}\n{}", path, content);
	// path.set_extension("index.css");
	fs::write(path, content)?;
	Ok(())
}
