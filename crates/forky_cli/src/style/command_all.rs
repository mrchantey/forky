use super::parse_to_file;
use anyhow::Result;
use clap::ArgMatches;
use forky_fs::terminal;
use forky_fs::Subcommand;
use forky_fs::WatchConfig;
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

		forky_fs::watch_path(
			&WatchConfig {
				watch: vec![glob::Pattern::new("**/*.css").unwrap()],
				ignore: vec![glob::Pattern::new("**/html/style.css").unwrap()],
				..Default::default()
			},
			|_| create_style_type_files().unwrap(),
		)?;
		// create_style_type_files()?;
		//TODO get all css files
		// create corresponding rust file
		// wrap in mod with same name as file
		// use marker, ie _g_.rs for .gitignore

		Ok(())
	}
}


fn create_style_type_files() -> Result<()> {
	glob("**/*_g.rs")
		.unwrap()
		.map(|path| fs::remove_file(path.unwrap()))
		.collect::<std::io::Result<()>>()?;

	glob("**/src/**/*.css")
		.unwrap()
		.filter_map(|val| val.ok())
		.map(|path_in| {
			let path_in = PathBuf::from(path_in);
			let parent = path_in.parent().unwrap_or_else(|| Path::new(""));
			let mut file_name = path_in.file_name().unwrap();
			file_name = Path::new(file_name).file_stem().unwrap();
			let mut file_name = file_name.to_os_string();
			file_name.push("_g");
			let path_out = parent.join(file_name).with_extension("rs");

			parse_to_file(
				path_in.to_str().unwrap(),
				path_out.to_str().unwrap(),
			)?;
			Ok(())
		})
		.collect::<Result<()>>()?;

	Ok(())
}
