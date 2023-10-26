use anyhow::Result;
use extend::ext;
use std::path::Path;
use std::path::PathBuf;

#[ext]
pub impl Path {
	fn relative(&self) -> Result<&Path> {
		let cwd = std::env::current_dir()?;
		let p = self.strip_prefix(cwd)?;
		Ok(p)
	}
	fn absolute(&self) -> Result<PathBuf> {
		let cwd = std::env::current_dir()?;
		let p = Path::join(&cwd, self);
		Ok(p)
	}
	fn to_forward_slash(&self) -> PathBuf {
		Path::new(&self.to_str().unwrap_or_default().replace("\\", "/"))
			.to_path_buf()
	}
	fn to_forward_slash_str(&self) -> String {
		self.to_str().unwrap_or_default().replace("\\", "/")
	}
}
