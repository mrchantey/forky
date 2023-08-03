use anyhow::Result;
use forky_fs::process::spawn_command_blocking;

pub struct Lightning {
	pub src: String,
	pub dst: String,
}
impl Lightning {
	#[rustfmt::skip]
	pub fn run(&self) -> Result<()>{
		println!("\nstyle: running lightningcss\n");
		let cmd = vec![
			"lightningcss", &self.src,
			"--bundle", 
			"--minify", 								
			"--nesting",						
			"--sourcemap",
			"-o", &self.dst
			];
		spawn_command_blocking(&cmd)
	}
}
