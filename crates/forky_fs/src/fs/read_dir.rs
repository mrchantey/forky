use crate::prelude::*;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Read a directory, returning nothing at all by default.
/// All options are opt-in.
#[derive(Debug)]
pub struct ReadDir {
	/// include files
	pub files: bool,
	/// include directories
	pub dirs: bool,
	/// search subdirectories
	pub recursive: bool,
	/// include the root directory
	pub root: bool,
}

impl Default for ReadDir {
	fn default() -> Self {
		Self {
			files: false,
			dirs: false,
			recursive: false,
			root: false,
		}
	}
}

impl ReadDir {
	/// Get all files and directories in a directory, not recursive
	pub fn all(root: impl AsRef<Path>) -> FsResult<Vec<PathBuf>> {
		Self {
			files: true,
			dirs: true,
			..Default::default()
		}
		.read(root)
	}

	/// Get all dirs in a directory, not recursive
	pub fn dirs(root: impl AsRef<Path>) -> FsResult<Vec<PathBuf>> {
		Self {
			dirs: true,
			..Default::default()
		}
		.read(root)
	}

	/// Get all files in a directory, not recursive
	pub fn files(root: impl AsRef<Path>) -> FsResult<Vec<PathBuf>> {
		Self {
			files: true,
			..Default::default()
		}
		.read(root)
	}

	/// Get all files and directories recursively
	pub fn all_recursive(root: impl AsRef<Path>) -> FsResult<Vec<PathBuf>> {
		Self {
			dirs: true,
			files: true,
			recursive: true,
			..Default::default()
		}
		.read(root)
	}

	/// Get all subdirectories recursively
	pub fn dirs_recursive(root: impl AsRef<Path>) -> FsResult<Vec<PathBuf>> {
		Self {
			dirs: true,
			recursive: true,
			..Default::default()
		}
		.read(root)
	}

	/// Get all files recursively
	pub fn files_recursive(root: impl AsRef<Path>) -> FsResult<Vec<PathBuf>> {
		Self {
			files: true,
			recursive: true,
			..Default::default()
		}
		.read(root)
	}


	/// Read dir with the provided options
	pub fn read(&self, root: impl AsRef<Path>) -> FsResult<Vec<PathBuf>> {
		let mut paths = Vec::new();
		if self.root {
			paths.push(root.as_ref().to_path_buf());
		}
		self.read_inner(root, &mut paths)?;
		Ok(paths)
	}
	fn read_inner(
		&self,
		dir: impl AsRef<Path>,
		paths: &mut Vec<PathBuf>,
	) -> FsResult<()> {
		let path = dir.as_ref();
		let children = fs::read_dir(path)
			.map_err(|e| FsError::from_io_with_dir(e, path))?;
		for child in children {
			let child = child.map_err(|e| FsError::Io(e)).map(|c| c.path())?;
			if child.is_dir() && self.dirs {
				paths.push(child.clone());
				if self.recursive {
					self.read_inner(child, paths)?;
				}
			} else if child.is_file() && self.files {
				paths.push(child.clone());
			} else {
				// ignore
			}
		}
		Ok(())
	}

	/// Read dir recursive for each path, ignoring DirNotFound errors
	pub fn read_dirs_ok(
		&self,
		paths: impl IntoIterator<Item = impl AsRef<Path>>,
	) -> FsResult<Vec<PathBuf>> {
		let mut vec = Vec::new();
		for path in paths {
			match self.read(path.as_ref()) {
				Ok(val) => {
					vec.extend(val);
				}
				Err(FsError::DirNotFound(_)) => {}
				Err(err) => return Err(err),
			};
		}
		Ok(vec)
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn fails() {
		expect(
			ReadDir::default()
				.read(FsExt::test_dir().join("foo"))
				.unwrap_err()
				.to_string(),
		)
		.to_contain("test_dir/foo");
	}
	#[test]
	fn dirs() {
		expect(
			ReadDir::dirs(FsExt::test_dir().join("foo"))
				.unwrap_err()
				.to_string(),
		)
		.to_contain("test_dir/foo");
		expect(ReadDir::dirs(FsExt::test_dir()).unwrap().len()).to_be(2);
	}
	#[test]
	fn read_dir_recursive() {
		expect(ReadDir::dirs_recursive(FsExt::test_dir()).unwrap().len())
			.to_be(2);
	}

	#[test]
	fn files() {
		expect(ReadDir::files(FsExt::test_dir()).unwrap().len()).to_be(3);
	}

	#[test]
	fn files_recursive() {
		expect(ReadDir::files_recursive(FsExt::test_dir()).unwrap().len())
			.to_be(3);
	}
}
