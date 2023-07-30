use anyhow::Result;
use forky_fs::fs::copy_recursive;
use forky_fs::process::spawn_command;
use std::path::Path;

pub fn run() -> Result<()> {
	copy_html()?;
	//loop
	cargo_run()?;
	// wasm_bingen()?;
	Ok(())
}

fn cargo_run() -> Result<()> {
	let cmd = "cargo build -p sweet --example test --release --target \
	           wasm32-unknown-unknown";
	spawn_command(&vec![cmd])?;
	// spawn_command(&vec!["cargo", "run", "--release"])?;
	Ok(())
}

pub fn wasm_bingen() -> Result<()> {
	let cmd = "wasm-bindgen --out-dir ./html/wasm --out-name bindgen --target \
	           web ./target/wasm32-unknown-unknown/release/examples/\
	           {{example}}.wasm";
	spawn_command(&vec![cmd])?;

	Ok(())
}

fn copy_html() -> Result<()> {
	let src = Path::new(&file!()).parent().unwrap().join("html_");
	let dst = Path::new(&"target/sweet-html");
	copy_recursive(src, dst)?;

	Ok(())
}
