// use super::*;
// use super::*;
// use clap::Arg;
// use std::fs;
use anyhow::Result;
use clap::ArgMatches;
use forky_fs::terminal;
use forky_fs::Subcommand;
use glob::*;
use std::fs;
use std::path::Path;
use std::path::PathBuf;


pub struct StyleCommandAll;

impl Subcommand for StyleCommandAll {
	fn name(&self) -> &'static str { "all" }
	fn about(&self) -> &'static str { "Apply to all style directories." }

	fn run(&self, _args: &ArgMatches) -> Result<()> {
		terminal::print_forky();
		glob("**/*_g.rs").unwrap().for_each(|path| {
			fs::remove_file(path.unwrap()).unwrap();
		});

		glob("**/src/**/*.css")
			.unwrap()
			.filter_map(|val| val.ok())
			.for_each(|path_in| {
				let path_in = PathBuf::from(path_in);
				let parent = path_in.parent().unwrap_or_else(|| Path::new(""));
				let mut file_name = path_in.file_name().unwrap();
				file_name = Path::new(file_name).file_stem().unwrap();
				let mut file_name = file_name.to_os_string();
				file_name.push("_g");
				let path_out = parent.join(file_name).with_extension("rs");

				println!("out: {:?}", path_out);
			});
		//TODO get all css files
		// create corresponding rust file
		// wrap in mod with same name as file
		// use marker, ie _g_.rs for .gitignore

		Ok(())
	}
}
