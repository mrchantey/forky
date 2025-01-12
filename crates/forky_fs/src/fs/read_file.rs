use super::FsError;
use super::FsResult;
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
}
