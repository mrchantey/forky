use super::*;
use anyhow::Result;
use colorize::*;
use forky_fs::fs::copy_recursive;
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

impl SweetCli {
	pub fn run(&self) -> Result<()> {
		terminal::clear();
		terminal::print_forky();


		let livereload = LiveReloadLayer::new();
		let reload = livereload.reloader();

		let server2 = self.server.clone();
		let _server_handle = std::thread::spawn(move || -> Result<()> {
			println!("Starting server at {}\n", server2.address.to_string_pretty());
			server2.serve_with_reload(livereload)
		});

		let kill = Arc::new(Mutex::new(()));
		let kill2 = kill.clone();
		let kill_unlocked = move || -> bool { kill2.try_lock().is_ok() };

		loop {
			self.copy_static()?;
			let kill2 = kill.clone();
			let change_listener = if self.watch {std::thread::spawn(move || -> Result<()> {
				let kill_lock = kill2.lock().unwrap();
				FsWatcher::default()
					.with_watch("**/*.rs")
					.with_ignore("**/target/**")
					.block()?;
				drop(kill_lock);
				Ok(())
			})}else{
				std::thread::spawn(move ||-> Result<()>{
					let _kill_lock = kill2.lock().unwrap();
loop {
		std::thread::sleep(Duration::from_secs(1))
}
// Ok(())
				})				
			};
			// let change_listener = std::thread::spawn(change_listener);
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
		println!("\n{success}\nServer running at {}\n", self.server.address.to_string_pretty());
	}

	#[rustfmt::skip]
	fn cargo_run(&self) -> Result<Child> {
		
		let mut cmd = vec![
			"cargo", "build",
			"--target", "wasm32-unknown-unknown",
		];
			
		if let Some(package) = &self.package {
			cmd.extend(vec!["-p", package]);
		};
		
		let example = self.example_name();
		cmd.extend(vec!["--example", &example]);
		if self.release {
				cmd.push("--release");
		}
		
		spawn_command(&cmd)
	}

	fn example_name(&self)->String{
		if let Some(package) = &self.package {
			format!("sweet_{}", package)
		}else{
			"sweet".to_string()
		}
	}

	#[rustfmt::skip]
	fn wasm_bingen(&self) -> Result<Child> {
		let mode = if self.release { "release" } else { "debug" };
		let example = self.example_name();
		let file = format!("target/wasm32-unknown-unknown/{mode}/examples/{example}.wasm");
		let hash = hash_file_to_string(&file)?;
		let out_file = format!("sweet-{hash}");
		self.replace_html_hash(&out_file)?;
		let cmd = vec![
			"wasm-bindgen", &file,
			"--no-typescript",
			"--target", "web",
			"--out-dir", &self.server.dir,
			"--out-name", &out_file,
		];
		spawn_command(&cmd)
	}

	fn copy_static(&self) -> Result<()> {
		let dst = Path::new(&self.server.dir);
		std::fs::remove_dir_all(&dst).ok();
		std::fs::create_dir_all(&dst)?;
		println!("copying runner files to {:?}", dst);

		std::fs::write(
			dst.join("index.html"),
			include_bytes!("html___/index.html"),
		)?;
		std::fs::write(
			dst.join("sweet-style.css"),
			include_bytes!("html___/sweet-style.css"),
		)?;

		if let Some(static_dir) = &self.static_dir {
			println!("copying static files from {:?}", static_dir);
			copy_recursive(static_dir, dst)?;
		}

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
