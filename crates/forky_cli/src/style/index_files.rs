use anyhow::Result;
use forky_core::PathBufX;
use forky_fs::fs::is_dir_or_extension;
use forky_fs::fs::read_dir_recursive;
use glob::*;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn create_index_files() -> Result<()> {
	glob("**/*.index.css")
		.unwrap()
		.map(|path| fs::remove_file(path.unwrap()))
		.collect::<std::io::Result<()>>()?;

	for_all_crates()?;
	// directories_matching("**/src/**/*.css")
	// 	.iter()
	// 	.for_each(|val| println!("dir: {:?}", val));

	Ok(())
}

fn for_all_crates() -> Result<()> {
	match fs::read_dir("crates") {
		Ok(dirs) => dirs.map(|e| e.unwrap().path()).for_each(|p| for_crate(p)),
		_ => for_crate(env::current_dir()?),
	}
	Ok(())
}

fn for_crate(path: PathBuf) {
	let path = PathBuf::push_with(&path, "src");
	read_dir_recursive(path)
		.into_iter()
		.map(|p| create_index_text(&p))
		.for_each(|p| println!("file: {:?}", p));
}
pub fn create_index_text(path: &PathBuf) -> String {
	fs::read_dir(&path)
		.unwrap()
		.map(|p| p.unwrap().path())
		// .filter(|c| !filename_included(c, IGNORE_FILES))
		.filter(|c| is_dir_or_extension(c, "css"))
		.map(|c| {
			let stem = c.file_stem().unwrap();
			let name = stem.to_str().unwrap().to_owned();
			if c.is_dir() {
				format!("@import './{name}/index.css';")
			} else {
				format!("@import './{name}.css';")
			}
		})
		.collect()
	// .fold(String::new(), |mut acc, val| {
	// 	acc.push_str(&val);
	// 	acc
	// })
}
