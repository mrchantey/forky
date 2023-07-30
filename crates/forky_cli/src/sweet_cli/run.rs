use anyhow::Result;
use forky_fs::fs::copy_recursive;
use forky_fs::process::spawn_command;
use std::path::Path;

const SRC_HTML_DIR: &str = "html___";
const DST_HTML_DIR: &str = "target/sweet-html";
const DST_CARGO_DIR: &str = "target/sweet-cargo";

pub struct SweetCliConfig {
	pub bin_name: String,
}
impl Default for SweetCliConfig {
	fn default() -> Self {
		Self {
			bin_name: "test".to_string(),
		}
	}
}


pub fn run(config: &SweetCliConfig) -> Result<()> {
	copy_html()?;
	//loop
	cargo_run(config)?;
	wasm_bingen(config)?;
	Ok(())
}

fn cargo_run(config: &SweetCliConfig) -> Result<()> {
	let cmd = vec![
		"cargo",
		"build",
		"-p",
		"sweet",
		"--example",
		&config.bin_name,
		"--release",
		"--out-dir",
		DST_CARGO_DIR,
		"--target",
		"wasm32-unknown-unknown",
		"-Z unstable-options",
	];
	spawn_command(&cmd)?;
	// spawn_command(&vec!["cargo", "run", "--release"])?;
	Ok(())
}

pub fn wasm_bingen(config: &SweetCliConfig) -> Result<()> {
	let cmd = "wasm-bindgen --out-dir ./html/wasm --out-name bindgen --target \
	           web ./target/wasm32-unknown-unknown/release/examples/test.wasm";
	spawn_command(&vec![cmd])?;

	Ok(())
}

fn copy_html() -> Result<()> {
	let src = Path::new(&file!()).parent().unwrap().join(SRC_HTML_DIR);
	let dst = Path::new(&DST_HTML_DIR);
	copy_recursive(src, dst)?;

	Ok(())
}
