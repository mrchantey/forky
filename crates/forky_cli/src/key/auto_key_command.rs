use super::*;
use anyhow::Result;
use clap::Parser;
use enigo::*;

/// automate keypresses
#[derive(Parser)]
pub struct AutoKeyCommand;

impl AutoKeyCommand {
	pub fn run(self) -> Result<()> {
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
