use super::*;
// use forky_core::ArcMut;
// use forky_fs::FsWatcher;
use forky_fs::Subcommand;

pub struct SweetCommand;

const ABOUT: &str = "build the wasm sweet runner and start a dev server";

impl Subcommand for SweetCommand {
	fn name(&self) -> &'static str { "sweet" }
	fn about(&self) -> &'static str { ABOUT }

	fn run(&self, _args: &clap::ArgMatches) -> anyhow::Result<()> {
		println!("sweet");
		run()
	}
}
