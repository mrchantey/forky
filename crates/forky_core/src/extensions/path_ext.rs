use anyhow::Result;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

pub struct PathExt;

impl PathExt {
	pub fn relative(path: &impl AsRef<Path>) -> Result<&Path> {
		let cwd = std::env::current_dir()?;
		let p = path.as_ref().strip_prefix(cwd)?;
		Ok(p)
	}
	pub fn absolute(path: impl AsRef<Path>) -> Result<PathBuf> {
		let cwd = std::env::current_dir()?;
		let p = Path::join(&cwd, path);
		Ok(p)
	}
	pub fn to_forward_slash(path: impl AsRef<Path>) -> PathBuf {
		Path::new(&path.as_ref().to_string_lossy().replace("\\", "/"))
			.to_path_buf()
	}
	pub fn to_forward_slash_str(path: impl AsRef<Path>) -> String {
		path.as_ref()
			.to_str()
			.unwrap_or_default()
			.replace("\\", "/")
	}

	pub fn file_stem(path: &impl AsRef<Path>) -> Result<&OsStr> {
		path.as_ref().file_stem().ok_or_else(|| {
			anyhow::anyhow!(
				"File has no stem: {}",
				path.as_ref().to_string_lossy()
			)
		})
	}
}
