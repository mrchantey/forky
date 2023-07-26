use cssparser::Parser;
use cssparser::ParserInput;
use cssparser::Token;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::*;
use std::collections::HashSet;
use std::fs;

pub fn get_stylesheet(path: proc_macro::TokenStream) -> String {
	let file_path = path.to_string();
	let file_path = file_path.trim_matches('"');
	fs::read_to_string(&file_path).expect("Expected to read file")
}

pub fn get_classes(css_content: &str) -> HashSet<String> {
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
	class_names
}
fn kebab_to_screaming_snake_case(input: &str) -> String {
	input
		.split('-')
		.collect::<Vec<_>>()
		.join("_")
		.to_uppercase()
}

pub fn classes_to_tokens(classes: HashSet<String>) -> TokenStream {
	let mut stream = TokenStream::new();
	for class in classes {
		let key = kebab_to_screaming_snake_case(&class);
		let ident = Ident::new(&key, proc_macro2::Span::call_site());
		let next = quote!(
			pub const #ident: &str = #class;
		);
		stream.append_all(next)
	}
	stream
}
