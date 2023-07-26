use anyhow::Result;
use cssparser::*;
use forky_core::StringX;
use std::collections::HashSet;
use std::fs;

pub fn parse(path: &str) -> String {
	let stylesheet = fs::read_to_string(path).expect("Expected to read file");
	let classes = get_classes(&stylesheet);
	let out = classes_to_rust(classes);
	out
}

pub fn parse_to_file(path_in: &str, path_out: &str) -> Result<()> {
	let out = parse(path_in);
	fs::write(path_out, out)?;
	Ok(())
}

pub fn get_classes(css_content: &str) -> Vec<String> {
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
fn kebab_to_screaming_snake_case(input: &str) -> String {
	input
		.split('-')
		.collect::<Vec<_>>()
		.join("_")
		.to_uppercase()
}

pub fn classes_to_rust(classes: Vec<String>) -> String {
	classes.iter().fold(String::new(), |mut acc, class| {
		let key = kebab_to_screaming_snake_case(&class);
		acc.push_string(&format!("pub const {key}: &str = \"{class}\";\n"));
		acc
	})
}

// pub fn classes_to_tokens(classes: HashSet<String>) -> TokenStream {
// 	let mut stream = TokenStream::new();
// 	for class in classes {
// 		let key = kebab_to_screaming_snake_case(&class);
// 		let ident = Ident::new(&key, proc_macro2::Span::call_site());
// 		let next = quote!(
// 			pub const #ident: &str = #class;
// 		);
// 		stream.append_all(next)
// 	}
// 	stream
// }
