use proc_macro2::Literal;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use std::iter::Peekable;

pub fn parse_config<I>(iter: &mut Peekable<I>) -> TokenStream
where
	I: Iterator<Item = TokenTree>,
{
	if let Some(t) = iter.peek() {
		match t {
			TokenTree::Ident(ident) => {
				let i_str = ident.to_string();
				match i_str.as_str() {
					"skip" => {
						let _ = iter.next().unwrap();
						return quote! {sweet::TestCaseConfig::Skip};
					}
					"only" => {
						let _ = iter.next().unwrap();
						return quote! {sweet::TestCaseConfig::Only};
					}
					_ => {}
				}
			}
			_ => {}
		}
	};
	quote! {sweet::TestCaseConfig::Default}
}


pub fn to_inventory_wrap_func(
	name: Literal,
	func: TokenStream,
	config: TokenStream,
) -> TokenStream {
	let func = quote!(||->anyhow::Result<()>{
		#func
		Ok(())
	});
	to_inventory(name, func, config)
}

pub fn to_inventory(
	name: Literal,
	func: TokenStream,
	config: TokenStream,
) -> TokenStream {
	quote!(
		inventory::submit!(sweet::TestCaseDesc {
			name: #name,
			func: #func,
			file: file!(),
			config: #config
		});
	)
}
