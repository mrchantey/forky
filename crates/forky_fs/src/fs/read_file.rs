use super::FsError;
use super::FsResult;
use std::fs;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;


/// A nicer read file that actually outputs the missing path
pub struct ReadFile;

impl ReadFile {
	pub fn to_string(path: impl AsRef<Path>) -> FsResult<String> {
		std::fs::read_to_string(&path)
			.map_err(|e| FsError::from_io_with_file(e, path))
	}
	pub fn to_bytes(path: impl AsRef<Path>) -> FsResult<Vec<u8>> {
		std::fs::read(&path).map_err(|e| FsError::from_io_with_file(e, path))
	}


	pub fn hash_file(path: impl AsRef<Path>) -> FsResult<u64> {
		let mut hasher = DefaultHasher::new();
		let file = Self::to_bytes(path)?;
		file.hash(&mut hasher);
		let hash = hasher.finish();
		Ok(hash)
	}

	/// Write a file, ensuring the path exists
	pub fn write(path: impl AsRef<Path>, data: &str) -> FsResult<()> {
		if let Some(parent) = path.as_ref().parent() {
			fs::create_dir_all(parent)?;
		}
		fs::write(path, data)?;
		Ok(())
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn to_string() {
		expect(ReadFile::to_string(FsExt::test_dir().join("mod.rs")).unwrap())
			.to_contain("pub mod included_dir;");

		expect(ReadFile::to_string(FsExt::test_dir().join("foo.rs")))
			.to_be_err();
	}

	#[test]
	fn to_bytes() {
		expect(
			ReadFile::to_bytes(FsExt::test_dir().join("mod.rs"))
				.unwrap()
				.len(),
		)
		.to_be_greater_than(10);

		expect(ReadFile::to_bytes(FsExt::test_dir().join("foo.rs")))
			.to_be_err();
	}


	#[test]
	fn hash() {
		let hash1 =
			ReadFile::hash_file(FsExt::test_dir().join("mod.rs")).unwrap();
		let hash2 =
			ReadFile::hash_file(FsExt::test_dir().join("included_file.rs"))
				.unwrap();
		expect(hash1).not().to_be(hash2);
	}
}
