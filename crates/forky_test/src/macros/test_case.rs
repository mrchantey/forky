use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, ReturnType};

pub struct TestCase {
	pub out: TokenStream,
}

impl TestCase {
	pub fn parse(_attr: TokenStream, input: TokenStream) -> TokenStream {
		let result = parse_macro_input!(input as TestCase).out;
		result
	}
}



impl Parse for TestCase {
	fn parse(stream: ParseStream) -> Result<Self> {
		let func = syn::ItemFn::parse(stream)?;
		let func_out = func.clone();
		let name = func.sig.ident;
		let name_str = name.to_string();

		println!("fuck");
		let func_name = match func.sig.output {
			ReturnType::Default => {
				let block = func.block;
				quote! {
					||->anyhow::Result{
							#block
							Ok(())
					}
				}
			}
			_ => name.to_token_stream(),
		};

		func.attrs.iter().for_each(|attr| {
			println!("attr: {}", attr.tokens);
		});

		let out = quote!(
			#func_out
			inventory::submit!(sweet::TestCaseDesc {
				name: #name_str,
				func: #func_name,
				file: file!(),
			});
		)
		.into();
		// let out = SuiteParser::parse(&mut stream);

		Ok(Self { out })
	}
}
