use std::io;
use std::path::Path;
use thiserror::Error;


/// An fs error that actuall outputs the missing path
#[derive(Debug, Error)]
pub enum FsError {
	#[error("file not found: {0}")]
	FileNotFound(String),
	#[error("dir not found: {0}")]
	DirNotFound(String),
	#[error("io error: {0}")]
	Io(io::Error),
}

impl FsError {
	pub fn assert_dir(path: impl AsRef<Path>) -> FsResult<()> {
		if !path.as_ref().is_dir() {
			Err(FsError::DirNotFound(
				path.as_ref().to_string_lossy().to_string(),
			))
		} else {
			Ok(())
		}
	}


	pub fn from_io(e: io::Error) -> Self { FsError::Io(e) }

	pub fn from_io_with_dir(e: io::Error, path: impl AsRef<Path>) -> Self {
		match e.kind() {
			io::ErrorKind::NotFound => FsError::DirNotFound(
				path.as_ref().to_string_lossy().to_string(),
			),
			_ => FsError::Io(e),
		}
	}
	pub fn from_io_with_file(e: io::Error, path: impl AsRef<Path>) -> Self {
		match e.kind() {
			io::ErrorKind::NotFound => FsError::FileNotFound(
				path.as_ref().to_string_lossy().to_string(),
			),
			_ => FsError::Io(e),
		}
	}

	pub fn file_not_found(path: impl AsRef<Path>) -> Self {
		FsError::FileNotFound(path.as_ref().to_string_lossy().to_string())
	}

	pub fn from_io_with_path(e: io::Error, path: impl AsRef<Path>) -> Self {
		let path = path.as_ref();
		if path.is_dir() {
			Self::from_io_with_dir(e, path)
		} else {
			Self::from_io_with_file(e, path)
		}
	}
}


impl From<io::Error> for FsError {
	fn from(e: io::Error) -> Self { FsError::from_io(e) }
}

pub type FsResult<T> = std::result::Result<T, FsError>;
