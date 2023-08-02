use super::*;
use crate::server::Server;
use anyhow::Result;
// use forky_core::*;
use forky_fs::fs::copy_recursive;
use forky_fs::process::spawn_command;
use forky_fs::process::ChildExt;
use forky_fs::process::ChildProcessStatus;
use forky_fs::FsWatcher;
use forky_fs::*;
use std::fs::DirEntry;
use std::path::Path;
use std::process::Child;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tower_livereload::LiveReloadLayer;
// use std::sync::Mutex;

const SRC_HTML_DIR: &str = "html___";
const DST_HTML_DIR: &str = "target/sweet";
const DST_HTML_FILE: &str = "target/sweet/index.html";
const DST_CARGO_DIR: &str = "target/sweet-tmp";

pub fn run(config: SweetCliConfig) -> Result<()> {
	copy_html()?;
	std::fs::create_dir_all(DST_CARGO_DIR)?;

	let port = 7777;

	let livereload = LiveReloadLayer::new();
	let reload = livereload.reloader();

	let _server_handle = std::thread::spawn(move || -> Result<()> {
		let server = Server {
			port,
			// clear: false,
			quiet: true,
			dir: DST_HTML_DIR.to_string(),
			..Default::default()
		};
		server.serve_with_reload(livereload)
	});

	let kill = Arc::new(Mutex::new(()));
	let kill2 = kill.clone();
	let kill_unlocked = move || -> bool { kill2.try_lock().is_ok() };

	println!("");
	loop {
		println!("Server running at http://127.0.0.1:{port}\n");

		let kill2 = kill.clone();
		let change_listener = std::thread::spawn(move || -> Result<()> {
			let kill_lock = kill2.blocking_lock();
			FsWatcher::default()
				.with_watch("**/*.rs")
				.with_ignore("target/**/*")
				.block()?;
			drop(kill_lock);
			Ok(())
		});
		// wait until fswatcher ready
		while kill_unlocked() {
			std::thread::sleep(Duration::from_millis(1))
		}

		match cargo_run(&config)?.wait_killable(kill_unlocked.clone()) {
			Ok(ChildProcessStatus::ExitSuccess(_)) => {}
			other => {
				eprintln!("sweet cli: cargo run failed: {:?}", other);
				change_listener.join().unwrap()?;
				continue;
			}
		}

		match wasm_bingen(&config)?.wait_killable(kill_unlocked.clone()) {
			Ok(ChildProcessStatus::ExitSuccess(_)) => {}
			other => {
				eprintln!("sweet cli: wasm bindgen failed: {:?}", other);
				change_listener.join().unwrap()?;
				continue;
			}
		}

		println!(
			"\nbuild succeeded!\nServer running at http://127.0.0.1:{port}"
		);

		reload.reload();
		change_listener.join().unwrap()?;
	}
}

#[rustfmt::skip]
fn cargo_run(config: &SweetCliConfig) -> Result<Child> {
	//TODO ideally this is done once, we need to remove the hash from the output file
	std::fs::remove_dir_all(DST_CARGO_DIR).ok();
	std::fs::create_dir_all(DST_CARGO_DIR)?;

	let tmp_out = DST_CARGO_DIR.to_string() + "/temp";
	println!("🤘 running cargo build");

	let mut cmd = vec![
		"cargo", "rustc",
		"--test", "sweet",
		"--target", "wasm32-unknown-unknown",
		"--release",
		];
		
		if let Some(package) = &config.package {
		cmd.extend(vec!["-p", package]);
	}

	cmd.extend(vec![
		"--", 
		"-o", &tmp_out, 
		"-Z", "unstable-options",
	]);
	spawn_command(&cmd)
}

#[rustfmt::skip]
pub fn wasm_bingen(_config: &SweetCliConfig) -> Result<Child> {
	let tmp_file = get_rustc_wasm()?;
	let path =  tmp_file.path();
	let stem = path.file_stem().unwrap();
	let path_str = path.to_str().unwrap();
	replace_html_hash(stem.to_str().unwrap())?;

	println!("\n🤘 running wasm bindgen for {path_str}..");
	let cmd = vec![
		"wasm-bindgen",
		"--out-dir", &DST_HTML_DIR,
		// "--out-name", "bindgen",
		"--target", "web",
		"--no-typescript",
		&path_str, // "./target/wasm32-unknown-unknown/release/examples/test.wasm",
	];
	spawn_command(&cmd)
}

fn copy_html() -> Result<()> {
	//TODO this may break when publishing, probably use file_abs!() instead
	let file = file_abs_workspace!();
	let dir = file.parent().unwrap();
	let src = dir.join(SRC_HTML_DIR);
	if !src.exists() {
		anyhow::bail!("src doesnt exist: {:?}", src);
	}
	let dst = Path::new(&DST_HTML_DIR);
	println!("copying files\nsrc: {:?}\ndst: {:?}", src, dst);
	std::fs::remove_dir_all(&dst).ok();
	copy_recursive(&src, &dst)?;

	Ok(())
}

fn get_rustc_wasm() -> Result<DirEntry> {
	let file = std::fs::read_dir(DST_CARGO_DIR)?
		.filter_map(|e| e.ok())
		.find(|e| e.file_name().to_str().unwrap().ends_with("wasm"));
	match file {
		Some(file) => Ok(file),
		None => anyhow::bail!("no wasm file found in {:?}", DST_CARGO_DIR),
	}
	// .ok()
	// .map(|e| e.path().to_str().unwrap().to_string());
}

fn replace_html_hash(name: &str) -> Result<()> {
	let html = std::fs::read_to_string(DST_HTML_FILE)?;
	let html = html.replace("__BINDGEN_FILE__", name);
	std::fs::write(DST_HTML_FILE, html)?;
	Ok(())
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
