use super::*;
use anyhow::Result;
use anyhow::bail;
use colorize::*;
use forky_fs::fs::copy_recursive;
use forky_fs::fs::hash_file_to_string;
use forky_fs::process::spawn_command;
use forky_fs::process::ChildExt;
use forky_fs::process::ChildProcessStatus;
use std::path::Path;
use std::process::Child;

impl SweetCli{
	pub fn build_wasm(&self,should_kill:impl Fn()->bool + Clone) -> Result<()> {
		self.copy_static()?;

		match self.cargo_run()
		.expect("\nCargo run failed\n")
		.wait_killable(should_kill.clone()) {
			Ok(ChildProcessStatus::ExitSuccess(_)) => {}
			other => {
				bail!("sweet cli: cargo run failed: {:?}", other);
			}
		}

		match self.wasm_bingen()
		.expect("\nWasm bindgen failed, try running `cargo install -f wasm-bindgen-cli`\n")
		.wait_killable(should_kill.clone()) {
			Ok(ChildProcessStatus::ExitSuccess(_)) => {}
			other => {
				bail!("sweet cli: wasm bindgen failed: {:?}", other);
			}
		}
		self.print_success();
		Ok(())
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
		
		cmd.extend(vec!["--example", &self.example]);
		if self.release {
				cmd.push("--release");
		}
		
		spawn_command(&cmd)
	}

	#[rustfmt::skip]
	fn wasm_bingen(&self) -> Result<Child> {
		let mode = if self.release { "release" } else { "debug" };
		let example = &self.example;
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