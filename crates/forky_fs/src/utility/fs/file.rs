use anyhow::Result;
use sha2::Digest;
use sha2::Sha256;
use std::fs;
use std::io;
use std::path::Path;


pub fn hash_file_to_bytes(path: impl AsRef<Path>) -> Result<Vec<u8>> {
	let mut hasher = Sha256::new();
	let mut file = fs::File::open(path)?;

	let _bytes_written = io::copy(&mut file, &mut hasher)?;
	let hash_bytes = hasher.finalize().to_vec();
	Ok(hash_bytes)
}

pub fn hash_file_to_string(path: impl AsRef<Path>) -> Result<String> {
	let mut hasher = Sha256::new();
	let mut file = fs::File::open(path)?;

	let _bytes_written = io::copy(&mut file, &mut hasher)?;
	let hash_bytes = hasher.finalize();
	let (hash_slice, _) = hash_bytes.split_at(8);
	let hash_str = hex::encode(hash_slice);
	Ok(hash_str)
}
