use super::Subcommand;
use crate::astro::{chart::Chart, serde::EarthMoment};
use anyhow::Result;
pub struct AstroCommand;

impl Subcommand for AstroCommand {
	fn name(&self) -> &'static str { "astro" }
	fn about(&self) -> &'static str { "get an astrology reading" }

	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> {
		vec![Box::new(Birthchart)]
	}
	fn run(&self, _args: &clap::ArgMatches) -> Result<()> {
		Birthchart.run(_args)
	}
}


struct Birthchart;

impl Subcommand for Birthchart {
	fn name(&self) -> &'static str { "Birthchart" }

	fn about(&self) -> &'static str { "Get your birthchart." }

	fn run(&self, _args: &clap::ArgMatches) -> Result<()> {
		let mom = match EarthMoment::from_disk() {
			Ok(value) => value,
			Err(_) => {
				let mom = EarthMoment::from_std()?;
				mom.to_disk()?;
				mom
			}
		};
		let chart = Chart::new(mom.into());
		println!("here is your chart:\n{}", chart.to_string());
		Ok(())
	}
}
