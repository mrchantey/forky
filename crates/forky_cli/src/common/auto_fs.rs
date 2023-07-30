use crate::auto_mod::AutoModCommand;
// use crate::style::StyleCommandAll;
use anyhow::Ok;
use anyhow::Result;
use clap::ArgMatches;
use forky_fs::Subcommand;
use std::sync::Arc;
use std::sync::Mutex;

pub struct AutoFs;

impl Subcommand for AutoFs {
	fn name(&self) -> &'static str { "auto-fs" }
	fn about(&self) -> &'static str { "generate mod and css files" }

	fn run(&self, _: &ArgMatches) -> Result<()> {
		// todo!("doesnt work,race condition, style removes css then mod doesnt see it");
		let mutex = Arc::new(Mutex::new(()));
		// let mutex1 = mutex.clone();
		let mutex2 = mutex.clone();

		// let handle1 =
		// 	std::thread::spawn(move || StyleCommandAll.run_with_mutex(mutex1));
		let handle2 =
			std::thread::spawn(move || AutoModCommand.run_with_mutex(mutex2));

		// handle1.join().unwrap()?;
		handle2.join().unwrap()?;
		Ok(())
	}
}
