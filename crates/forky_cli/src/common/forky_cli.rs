use crate::prelude::*;
use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

/// Welcome to the Forky Cli!
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ForkyCli {
	#[command(subcommand)]
	command: Commands,
}
#[derive(Subcommand)]
enum Commands {
	AutoKey(AutoKeyCommand),
	Fed(ForEachDirectory),
	Mod(AutoModCommand),
	Serve(Serve),
	Watch(WatchCommand),
}

impl ForkyCli {
	pub fn run() -> Result<()> {
		match Self::parse().command {
			Commands::AutoKey(cmd) => cmd.run(),
			Commands::Fed(cmd) => cmd.run(),
			Commands::Mod(cmd) => cmd.run(),
			Commands::Serve(cmd) => cmd.run(),
			Commands::Watch(cmd) => cmd.run(),
		}
	}
}
