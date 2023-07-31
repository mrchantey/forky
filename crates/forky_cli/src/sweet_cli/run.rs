use crate::server::Server;
use anyhow::Result;
use forky_core::OptionTExt;
use forky_core::PathX;
use forky_fs::fs::copy_recursive;
use forky_fs::process::spawn_command;
use forky_fs::FsWatcher;
use forky_fs::*;
use std::fmt::Display;
use std::path::Path;

const SRC_HTML_DIR: &str = "html___";
const DST_HTML_DIR: &str = "target/sweet";
const DST_CARGO_DIR: &str = "target/sweet-tmp";

#[derive(Debug, Clone)]
pub struct SweetCliConfig {
	pub package: Option<String>,
}
impl Default for SweetCliConfig {
	fn default() -> Self { Self { package: None } }
}

impl Display for SweetCliConfig {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(package) = &self.package {
			write!(f, "package: {package}")?;
		}
		Ok(())
	}
}


pub fn run(config: SweetCliConfig) -> Result<()> {
	copy_html()?;
	std::fs::create_dir_all(DST_CARGO_DIR)?;

	let port = 7777;

	let handle_build =
		std::thread::spawn(move || {
			FsWatcher::default().with_watch("**/*.rs").watch(|_| {
				println!("building..\n{config}\n");
				cargo_run(&config)?;
				wasm_bingen(&config)?;
				println!("\nbuild succeeded!\nServer running at http://127.0.0.1:{port}");
				Ok(())
			})
		});

	let handle_serve = std::thread::spawn(move || -> Result<()> {
		let _server = Server {
			port,
			quiet: true,
			dir: DST_HTML_DIR.to_string(),
			..Default::default()
		}
		.serve_forever()?;
		Ok(())
	});

	//creates race condition if all files arent copied at exactly the same time
	handle_build.join().unwrap()?;
	handle_serve.join().unwrap()?;

	Ok(())
}

fn cargo_run(config: &SweetCliConfig) -> Result<()> {
	let tmp_out = DST_CARGO_DIR.to_string() + "/temp";
	println!("running cargo build");

	let mut cmd = vec![
		"cargo",
		"rustc",
		"--test",
		"sweet",
		"--release",
		"--target",
		"wasm32-unknown-unknown",
	];
	if let Some(package) = &config.package {
		cmd.push("-p");
		cmd.push(package);
	}
	cmd.extend(vec![
		"--",
		"-o",
		&tmp_out,
		// "target/sweet-foo",
		"-Z",
		"unstable-options",
	]);

	// let cmd = vec![
	// 	"cargo",
	// 	"rustc",
	// 	"-p",
	// 	"sweet",
	// 	// "--example",
	// 	// &config.bin_name,
	// 	"-Z unstable-options",
	// 	"--out-dir",
	// 	&DST_CARGO_DIR,
	// 	// "--",
	// 	// "-o",
	// 	// &tmp_out,
	// 	// DST_HTML_DIR,
	// ];
	spawn_command(&cmd)?;
	Ok(())
}

pub fn wasm_bingen(_config: &SweetCliConfig) -> Result<()> {
	let tmp_file = get_rustc_wasm()?;

	println!("running wasm bindgen for {tmp_file}..");
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
	//TODO this may break when publishing, probably use file_abs!() instead
	let file = file_abs_workspace!();
	let dir = file.parent().unwrap();
	let src = dir.join(SRC_HTML_DIR);
	if !src.exists() {
		panic!("src doesnt exist: {:?}", src);
	}
	let dst = Path::new(&DST_HTML_DIR);
	println!("copying files\nsrc: {:?}\ndst: {:?}", src, dst);
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


/*

	let cmd = vec![
		"cargo",
		"build",
		"rustc",
		"-p",
		"sweet",
		"--test",
		"sweet",
		// "--example",
		// &config.bin_name,
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


*/
