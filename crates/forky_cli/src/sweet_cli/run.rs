use super::*;
use anyhow::Result;
use colorize::*;
use forky_fs::fs::hash_file_to_string;
use forky_fs::process::spawn_command;
use forky_fs::process::ChildExt;
use forky_fs::process::ChildProcessStatus;
use forky_fs::terminal;
use forky_fs::FsWatcher;
use std::path::Path;
use std::process::Child;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tower_livereload::LiveReloadLayer;
// use std::sync::Mutex;

// const DST_CARGO_DIR: &str = "target/sweet-tmp";

impl SweetCli {
	pub fn run(&self) -> Result<()> {
		terminal::clear();
		terminal::print_forky();


		let livereload = LiveReloadLayer::new();
		let reload = livereload.reloader();

		let server2 = self.server.clone();
		let _server_handle = std::thread::spawn(move || -> Result<()> {
			println!("Starting server at {}\n", server2.address);
			server2.serve_with_reload(livereload)
		});

		let kill = Arc::new(Mutex::new(()));
		let kill2 = kill.clone();
		let kill_unlocked = move || -> bool { kill2.try_lock().is_ok() };

		// copy html just before wasm-bindgen so runner still works during a new build
		let mut first_run = true;
		loop {
			let kill2 = kill.clone();
			let change_listener = std::thread::spawn(move || -> Result<()> {
				let kill_lock = kill2.lock().unwrap();
				FsWatcher::default()
					.with_watch("**/*.rs")
					.with_ignore("target")
					.block()?;
				drop(kill_lock);
				Ok(())
			});
			// wait until fswatcher ready
			while kill_unlocked() {
				std::thread::sleep(Duration::from_millis(1))
			}

			match self.cargo_run()?.wait_killable(kill_unlocked.clone()) {
				Ok(ChildProcessStatus::ExitSuccess(_)) => {}
				other => {
					eprintln!("sweet cli: cargo run failed: {:?}", other);
					change_listener.join().unwrap()?;
					continue;
				}
			}

			if first_run {
				self.copy_html()?;
				first_run = false;
			}

			match self.wasm_bingen()?.wait_killable(kill_unlocked.clone()) {
				Ok(ChildProcessStatus::ExitSuccess(_)) => {}
				other => {
					eprintln!("sweet cli: wasm bindgen failed: {:?}", other);
					change_listener.join().unwrap()?;
					continue;
				}
			}
			self.print_success();
			reload.reload();
			change_listener.join().unwrap()?;
		}
	}

	fn print_success(&self) {
		let success = "Build succeeded".b_green().bold();
		println!("\n{success}\nServer running at {}\n", self.server.address);
	}

	#[rustfmt::skip]
	fn cargo_run(&self) -> Result<Child> {
		println!("running cargo build\n");

		let mut cmd = vec![
			"cargo", "build",
			"--example", "sweet",
			"--target", "wasm32-unknown-unknown",
			];

		if self.release {
				cmd.push("--release");
		}
		if let Some(package) = &self.package {
			cmd.extend(vec!["-p", package]);
		}
		spawn_command(&cmd)
	}

	#[rustfmt::skip]
	fn wasm_bingen(&self) -> Result<Child> {
		let file = "target/wasm32-unknown-unknown/debug/examples/sweet.wasm";
		let hash = hash_file_to_string(file)?;
		let out_file = format!("sweet-{hash}");
		self.replace_html_hash(&out_file)?;
		println!("\nrunning wasm bindgen for {out_file}");
		let cmd = vec![
			"wasm-bindgen", &file,
			"--out-dir", &self.server.dir,
			"--out-name", &out_file,
			"--target", "web",
			"--no-typescript",
		];
		spawn_command(&cmd)
	}

	fn copy_html(&self) -> Result<()> {
		let dst = Path::new(&self.server.dir);
		std::fs::remove_dir_all(&dst).ok();
		std::fs::create_dir_all(&dst)?;
		println!("copying html to {:?}", dst);

		std::fs::write(
			dst.join("index.html"),
			include_bytes!("html___/index.html"),
		)?;
		std::fs::write(
			dst.join("style.css"),
			include_bytes!("html___/style.css"),
		)?;

		Ok(())
	}

	fn replace_html_hash(&self, name: &str) -> Result<()> {
		let file = Path::new(&self.server.dir).join("index.html");
		let html = std::fs::read_to_string(&file)?;
		let html = html.replace("__BINDGEN_FILE__", name);
		std::fs::write(&file, &html)?;
		Ok(())
	}
}
