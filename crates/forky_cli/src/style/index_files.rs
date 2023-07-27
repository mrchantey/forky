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
		Ok(dirs) => dirs
			.map(|e| e.unwrap().path())
			.map(|p| for_crate(p))
			.collect::<Result<()>>()?,
		_ => for_crate(env::current_dir()?)?,
	}
	Ok(())
}

fn for_crate(path: PathBuf) -> Result<()> {
	let path = PathBuf::push_with(&path, "src");
	read_dir_recursive(path)
		.into_iter()
		//TODO filter by directories that contain any css
		.map(|p| (create_index_text(&p).unwrap(), p))
		.map(|(content, path)| save_to_disk(&path, content))
		.collect()
}

// this looks slow
fn dir_contains_css(path: &PathBuf) -> bool {
	let pattern = Pattern::new("**/*.css").unwrap();
	glob::glob(&pattern.to_string())
		.unwrap()
		.filter_map(|val| val.ok())
		.any(|val| val.parent().unwrap() == path)
}

pub fn create_index_text(path: &PathBuf) -> Result<String> {
	let ignore_files = Pattern::new("**/*/index.css").unwrap();

	let out = fs::read_dir(&path)
		.unwrap()
		.map(|p| p.unwrap().path())
		.filter(|c| !ignore_files.matches(c.to_str().unwrap()))
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
		.collect();
	Ok(out)
}


fn save_to_disk(path: &PathBuf, content: String) -> Result<()> {
	// let mut path = path.clone();
	println!("path: {:?}\n{}", path, content);
	// path.set_extension("index.css");
	// fs::write(path, content)?;
	Ok(())
}
