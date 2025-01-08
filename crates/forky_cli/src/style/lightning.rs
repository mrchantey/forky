use anyhow::Result;
use forky_core::prelude::*;
use forky_fs::prelude::CommandExt;
// use forky_fs::process::spawn_command_blocking;
use std::fs::create_dir_all;
use std::path::Path;

pub struct Lightning {
	pub src: String,
	pub dst: String,
}
impl Lightning {
	#[rustfmt::skip]
	pub fn run(&self) -> Result<()>{
		create_dir_all(Path::new(&self.dst).parent().ok()?)?;

		println!("\nstyle: running lightningcss\n");
		let cmd = vec![
			"npx", "lightningcss", &self.src,
			"--bundle", 
			"--minify", 								
			"--nesting",						
			"--sourcemap",
			"-o", &self.dst
			];
		// let cmd = vec![
		// 	"lightningcss"
		// 	];
		// spawn_command_blocking(&cmd)
		CommandExt::spawn_command_with_shell_blocking(&cmd)
	}
}
