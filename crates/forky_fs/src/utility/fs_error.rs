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
	pub fn from_io(e: io::Error) -> Self { FsError::Io(e) }

	pub fn from_io_with_path(path: impl AsRef<Path>, e: io::Error) -> Self {
		let path = path.as_ref();
		match (e.kind(), path.is_dir()) {
			(io::ErrorKind::NotFound, true) => {
				FsError::DirNotFound(path.to_string_lossy().to_string())
			}
			(io::ErrorKind::NotFound, false) => {
				FsError::FileNotFound(path.to_string_lossy().to_string())
			}
			_ => FsError::Io(e),
		}
	}
}


impl From<io::Error> for FsError {
	fn from(e: io::Error) -> Self { FsError::from_io(e) }
}

pub type FsResult<T> = std::result::Result<T, FsError>;
