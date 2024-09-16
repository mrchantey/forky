use super::*;
use anyhow::Result;
use clap::Arg;
use clap::ArgMatches;
use forky_fs::prelude::*;
use std::path::Path;

pub struct StyleCommandFile;

impl Subcommand for StyleCommandFile {
	fn name(&self) -> &'static str { "file" }
	fn about(&self) -> &'static str { "Apply to specified style file." }

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command
			.arg(Arg::new("in").required(true))
			.arg(Arg::new("out").required(false))
	}
	fn run(&self, args: &ArgMatches) -> Result<()> {
		terminal::print_forky();
		let path_in = args.get_one::<String>("in").unwrap();
		let path_in = Path::new(path_in).to_path_buf();
		if !path_in.exists() {
			anyhow::bail!("path not found: {:?}", path_in)
		}
		let content = create_type_text(&path_in);
		let path_out = if let Some(path_out) = args.get_one::<String>("out") {
			Path::new(path_out).to_path_buf()
		} else {
			super::type_files::get_path_out(&path_in)
		};
		super::type_files::write_to_disk(&path_out, &content)?;
		Ok(())
	}
}
