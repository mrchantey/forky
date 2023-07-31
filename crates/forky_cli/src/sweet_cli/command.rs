use super::*;
use forky_fs::Subcommand;
// use forky_core::ArcMut;
use forky_fs::*;

pub struct SweetCommand;

const ABOUT: &str = "build the wasm sweet runner and start a dev server";

impl Subcommand for SweetCommand {
	fn name(&self) -> &'static str { "sweet" }
	fn about(&self) -> &'static str { ABOUT }

	fn run(&self, _args: &clap::ArgMatches) -> anyhow::Result<()> {
		terminal::clear();
		terminal::print_forky();
		println!("sweet");
		let config = SweetCliConfig::default();
		run(config)
	}
}
