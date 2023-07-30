use anyhow::Result;
use forky_fs::fs::copy_recursive;
use std::path::Path;

pub fn run() -> Result<()> {
	copy_html()?;
	Ok(())
}


fn copy_html() -> Result<()> {
	let src = Path::new(&file!()).parent().unwrap().join("html");
	let dst = Path::new(&"target/sweet-html");
	copy_recursive(src, dst)?;

	Ok(())
}
