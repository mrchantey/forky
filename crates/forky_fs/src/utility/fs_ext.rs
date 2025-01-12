use crate::prelude::*;
use glob::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;


/// Better Fs, actually outputs missing path
pub struct FsExt;

impl FsExt {
	/// Read dir and return children, collecting all errors
	pub fn read_dir(path: impl AsRef<Path>) -> FsResult<Vec<fs::DirEntry>> {
		let children = Self::read_dir_raw(path)?
			.into_iter()
			.map(|c| c.map_err(|e| FsError::Io(e)))
			.collect::<FsResult<Vec<_>>>()?;
		Ok(children)
	}

	/// Read dir recursive for each path, ignoring any not found dirs
	pub fn read_dir_recursive_some(
		paths: impl IntoIterator<Item = impl AsRef<Path>>,
	) -> FsResult<Vec<PathBuf>> {
		let mut vec = Default::default();
		for path in paths {
			match Self::read_dir_recursive_inner(&mut vec, path.as_ref()) {
				Err(FsError::DirNotFound(_)) => Ok(()),
				Ok(_) => Ok(()),
				Err(err) => Err(err),
			}?;
		}
		Ok(vec)
	}

	/// Read dir returing children that may be an error
	pub fn read_dir_raw(path: impl AsRef<Path>) -> FsResult<fs::ReadDir> {
		let path = path.as_ref();
		fs::read_dir(path).map_err(|e| FsError::from_io_with_dir(e, path))
	}

	/// read directory and return all sub directories, files are ignored
	pub fn read_dir_recursive(
		path: impl AsRef<Path>,
	) -> FsResult<Vec<PathBuf>> {
		FsError::assert_dir(&path)?;
		let mut vec = Default::default();
		Self::read_dir_recursive_inner(&mut vec, &path)?;
		Ok(vec)
	}
	fn read_dir_recursive_inner(
		arr: &mut Vec<PathBuf>,
		path: impl AsRef<Path>,
	) -> FsResult<()> {
		let path = path.as_ref();
		if !path.is_dir() {
			return Ok(());
		}
		arr.push(path.to_path_buf());
		for child in Self::read_dir(path)? {
			let child_path = child.path();
			if child
				.file_type()
				.map_err(|e| FsError::from_io_with_file(e, &child_path))?
				.is_dir()
			{
				Self::read_dir_recursive_inner(arr, &child_path)?;
			}
		}
		Ok(())
	}

	/// get all directories matching a glob pattern, removing duplicates
	pub fn directories_matching(pattern: &str) -> Vec<PathBuf> {
		glob(pattern)
			.unwrap()
			.filter_map(|val| val.ok())
			.map(|val| {
				if val.is_dir() {
					val
				} else {
					val.parent().unwrap().to_path_buf()
				}
			})
			.collect::<HashSet<PathBuf>>()
			// .fold(HashSet::new(), |mut acc, val| {
			// 	acc.insert(val);
			// 	acc
			// })
			.iter()
			.map(|path| path.clone())
			.collect::<Vec<_>>()
	}

	/// Copy a directory recursively,
	/// creating it if it doesnt exist
	pub fn copy_recursive(
		source: impl AsRef<Path>,
		destination: impl AsRef<Path>,
	) -> FsResult<()> {
		let source = source.as_ref();
		let destination = destination.as_ref();

		fs::create_dir_all(&destination).ok();
		for entry in Self::read_dir(source)? {
			// let entry =
			// entry.map_err(|e| FsError::from_io(entry, e))?;
			// let path = entry.path();
			if entry.file_type().map(|f| f.is_dir()).unwrap_or(false) {
				Self::copy_recursive(
					entry.path(),
					destination.join(entry.file_name()),
				)?;
			} else {
				fs::copy(entry.path(), destination.join(entry.file_name()))
					.map_err(FsError::from_io)?;
			}
		}
		Ok(())
	}

	// pub fn dir_contains(path: PathBuf, pattern: &str) -> bool {
	// 	let pattern = Pattern::new(pattern).unwrap();
	// 	glob::glob_with(
	// 		&pattern.to_string(),
	// 		glob::MatchOptions {
	// 			case_sensitive: false,
	// 			require_literal_separator: false,
	// 			require_literal_leading_dot: false,
	// 		},
	// 	)
	// 	read_dir_recursive(path)
	// 		.iter()
	// 		.any(|p| pattern. p.to_str().unwrap().contains(pattern))
	// }

	pub fn hash_file(path: impl AsRef<Path>) -> io::Result<u64> {
		let mut hasher = DefaultHasher::new();
		let mut file = fs::File::open(path)?;

		let mut buffer = Vec::new();
		file.read_to_end(&mut buffer)?;
		buffer.hash(&mut hasher);

		let hash = hasher.finish();
		Ok(hash)
	}



	pub fn write<P>(filename: P, data: &str) -> io::Result<()>
	where
		P: AsRef<Path>,
	{
		let mut file = File::create(filename)?;
		file.write_all(data.as_bytes())?;
		Ok(())
	}

	/// Search up the directory until a `Cargo.lock` is found
	pub fn project_root() -> FsResult<PathBuf> {
		let path = std::env::current_dir()?;
		let mut path_ancestors = path.as_path().ancestors();

		while let Some(p) = path_ancestors.next() {
			let has_cargo = Self::read_dir(p)?
				.into_iter()
				.any(|p| p.file_name() == OsString::from("Cargo.lock"));
			if has_cargo {
				return Ok(PathBuf::from(p));
			}
		}
		Err(FsError::FileNotFound(
			"Failed to find project root".to_string(),
		))
	}
}


#[cfg(test)]
mod test {
	use super::FsExt;
	use std::path::Path;
	use std::path::PathBuf;
	use sweet::prelude::*;


	const NUM_CRATES: usize = 8;

	fn from_root(path: impl AsRef<Path>) -> PathBuf {
		FsExt::project_root().unwrap().join(path)
	}

	#[test]
	fn project_root() {
		expect(FsExt::project_root()).to_be_ok();
		expect(FsExt::project_root().unwrap().join("Cargo.lock").exists())
			.to_be_true();
	}

	#[test]
	fn read_dir() {
		expect(FsExt::read_dir("").unwrap_err().to_string())
			.to_be("dir not found: ");
		expect(FsExt::read_dir("./foobar").unwrap_err().to_string())
			.to_be("dir not found: ./foobar");
		expect(FsExt::read_dir(from_root("crates")).unwrap().len())
			.to_be(NUM_CRATES);
	}
	#[test]
	fn read_dir_recursive() {
		expect(FsExt::read_dir_recursive("").unwrap_err().to_string())
			.to_be("dir not found: ");
		expect(
			FsExt::read_dir_recursive(from_root("crates"))
				.unwrap()
				.len(),
		)
		.to_be_greater_than(NUM_CRATES);


		let entries = FsExt::read_dir_recursive(from_root("crates")).unwrap();
		println!("hello {:#?}", entries);
	}
}
