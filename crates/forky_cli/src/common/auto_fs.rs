use crate::auto_mod::AutoModCommand;
use crate::style::StyleCommandAll;
use anyhow::Ok;
use anyhow::Result;
use forky_fs::Subcommand;

pub struct AutoFs;

impl Subcommand for AutoFs {
	fn name(&self) -> &'static str { "auto-fs" }
	fn about(&self) -> &'static str { "generate mod and css files" }

	fn run(&self, args: &clap::ArgMatches) -> Result<()> {
		// todo!("doesnt work,race condition, style removes css then mod doesnt see it");
		let args1 = args.clone();
		let args2 = args.clone();
		let handle1 = std::thread::spawn(move || StyleCommandAll.run(&args1));
		let handle2 = std::thread::spawn(move || AutoModCommand.run(&args2));

		handle1.join().unwrap()?;
		handle2.join().unwrap()?;
		Ok(())
	}
}
