use super::*;
use anyhow::Result;
use enigo::*;
use forky_fs::prelude::*;
pub struct AutoKeyCommand;


impl Subcommand for AutoKeyCommand {
	fn name(&self) -> &'static str { "key" }
	fn about(&self) -> &'static str { "automate keypresses" }

	fn run(&self, _args: &clap::ArgMatches) -> Result<()> {
		InputSequence::default()
			// exit dock
			.input(Key::Escape)
			.ulaunch(1, "r beetmash-biz")
			.ulaunch(2, "r beetmash-api")
			.ulaunch(3, "r beetmash-site")
			.ulaunch(4, "r beetmash")
			.ulaunch(5, "r beet")
			// .command("code /home/pete/me/beetmash-biz")
			// .command("cd ~ && code .")
			.run()?;
		Ok(())
	}
}
