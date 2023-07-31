use crate::server::Server;
use anyhow::Result;
use forky_core::OptionTExt;
use forky_fs::fs::copy_recursive;
use forky_fs::process::spawn_command;
use forky_fs::FsWatcher;
use std::path::Path;

const SRC_HTML_DIR: &str = "html___";
const DST_HTML_DIR: &str = "target/sweet";
const DST_CARGO_DIR: &str = "target/sweet-tmp";

#[derive(Debug, Clone)]
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


pub fn run(config: SweetCliConfig) -> Result<()> {
	copy_html()?;
	//loop
	let handle_build = std::thread::spawn(move || {
		FsWatcher::default().with_watch("**/*.rs").watch(|_| {
			cargo_run(&config)?;
			wasm_bingen(&config)?;
			Ok(())
		})
	});

	let handle_serve = std::thread::spawn(move || {
		Server::serve_forever(|| Server::default().with_dir(DST_HTML_DIR))
	});

	handle_build.join().unwrap()?;
	handle_serve.join().unwrap()?;

	Ok(())
}

fn cargo_run(config: &SweetCliConfig) -> Result<()> {
	// let tmp_out = DST_CARGO_DIR.to_string() + "/temp";
	println!("running cargo rustc");
	let cmd = vec![
		"cargo",
		"build",
		// "rustc",
		"-p",
		"sweet",
		"--example",
		&config.bin_name,
		"--release",
		"--target",
		"wasm32-unknown-unknown",
		"-Z unstable-options",
		"--out-dir",
		&DST_CARGO_DIR,
		// "--",
		// "-o",
		// &tmp_out,
		// DST_HTML_DIR,
	];
	spawn_command(&cmd)?;
	Ok(())
}

pub fn wasm_bingen(_config: &SweetCliConfig) -> Result<()> {
	let tmp_file = get_rustc_wasm()?;

	println!("running wasm bindgen for {tmp_file}");
	let cmd = vec![
		"wasm-bindgen",
		"--out-dir",
		&DST_HTML_DIR,
		"--out-name",
		"bindgen",
		"--target",
		"web",
		"--no-typescript",
		&tmp_file, // "./target/wasm32-unknown-unknown/release/examples/test.wasm",
	];
	spawn_command(&cmd)?;
	Ok(())
}

fn copy_html() -> Result<()> {
	let src = Path::new(&file!()).parent().unwrap().join(SRC_HTML_DIR);
	let dst = Path::new(&DST_HTML_DIR);
	copy_recursive(src, dst)?;

	Ok(())
}


fn get_rustc_wasm() -> Result<String> {
	std::fs::read_dir(DST_CARGO_DIR)?
		.filter_map(|e| e.ok())
		.find(|e| e.file_name().to_str().unwrap().ends_with("wasm"))
		.ok()
		.map(|e| e.path().to_str().unwrap().to_string())
}
