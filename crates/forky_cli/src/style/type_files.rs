use anyhow::Result;
use cssparser::*;
use forky_core::prelude::*;
use glob::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn create_type_files() -> Result<()> {
	println!("\nstyle: creating type files...\n");
	remove_all()?;
	create_all()?;
	Ok(())
}

fn remove_all() -> Result<()> {
	glob("**/*_g.rs")
		.unwrap()
		.map(|path| fs::remove_file(path.unwrap()))
		.collect::<std::io::Result<()>>()?;
	Ok(())
}

fn create_all() -> Result<()> {
	let ignore = Pattern::new("**/target/**").unwrap();

	glob("**/src/**/*.css")
		.unwrap()
		.filter_map(|val| val.ok())
		.filter(|p| !p.parent_ends_with_underscore())
		.filter(|p| !ignore.matches_path(p))
		.filter(|path| {
			let stem = path.file_stem().unwrap();
			stem != "index" && stem != "lib"
		})
		.map(|path_in| (create_type_text(&path_in), path_in))
		.map(|(content, path_in)| {
			write_to_disk(&get_path_out(&path_in), &content)
		})
		.collect::<Result<()>>()
}

pub fn create_type_text(path: &PathBuf) -> String {
	let path = path.to_str().unwrap();
	let stylesheet = fs::read_to_string(path).expect("Expected to read file");
	let classes = get_classes(&stylesheet);
	let file_name = Path::new(path).file_stem().unwrap();
	let file_name = file_name.to_string_lossy();
	let classes = classes_to_rust(classes);
	format!("pub mod {} {{\n{}}}", file_name, classes)
}

pub fn write_to_disk(path_out: &PathBuf, content: &str) -> Result<()> {
	fs::write(path_out, content)?;
	println!("style: created: {:?}", path_out);
	Ok(())
}

pub fn get_path_out(path_in: &PathBuf) -> PathBuf {
	let parent = path_in.parent().unwrap_or_else(|| Path::new(""));
	let mut file_name = path_in.file_name().unwrap();
	file_name = Path::new(file_name).file_stem().unwrap();
	let mut file_name = file_name.to_os_string();
	file_name.push("_g");
	parent.join(file_name).with_extension("rs")
}


fn get_classes(css_content: &str) -> Vec<String> {
	let mut class_names = HashSet::new();

	let mut parser_input = ParserInput::new(&css_content);
	let mut parser = Parser::new(&mut parser_input);

	while let Ok(token) = parser.next() {
		match token {
			Token::Delim(val) => {
				if val.to_string() == "." {
					if let Ok(token) = parser.next() {
						if let Token::Ident(class) = token {
							// println!("found class: {}", class.to_string());
							class_names.insert(class.to_string());
						}
					}
				}
			}
			_ => (),
		}
	}
	let mut class_names = class_names.into_iter().collect::<Vec<_>>();
	class_names.sort();
	class_names
}

fn classes_to_rust(classes: Vec<String>) -> String {
	classes.iter().fold(String::new(), |mut acc, class| {
		let key = kebab_to_screaming_snake_case(&class);
		acc.push_string(&format!("\tpub const {key}: &str = \"{class}\";\n"));
		acc
	})
}

fn kebab_to_screaming_snake_case(input: &str) -> String {
	input
		.split('-')
		.collect::<Vec<_>>()
		.join("_")
		.to_uppercase()
}
