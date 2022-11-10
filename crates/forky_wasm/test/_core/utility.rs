use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;


pub fn read_wasm_bytes(path: &str) -> io::Result<(Vec<u8>)> {
	let path = format!(
		"../../target/wasm32-unknown-unknown/release/wasm_{}.wasm",
		path
	);
	let f = File::open(path)?;
	// .err(panic!(format!("could not find path: {}", path)));
	let mut reader = BufReader::new(f);
	let mut buffer = Vec::new();
	reader.read_to_end(&mut buffer)?;
	Ok(buffer)
}
