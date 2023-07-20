use super::*;
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::parse_macro_input;

pub struct TestCaseAttr {
	pub out: TokenStream,
}

impl TestCaseAttr {
	pub fn parse(_attr: TokenStream, input: TokenStream) -> TokenStream {
		let result = parse_macro_input!(input as TestCaseAttr).out;
		result
	}
}

impl Parse for TestCaseAttr {
	fn parse(stream: ParseStream) -> Result<Self> {
		let func = syn::ItemFn::parse(stream)?;
		let func_out = func.clone();
		let name = func.sig.ident;
		let name_str = name.to_string();
		let func_ident = name.to_token_stream();
		let submit = to_inventory(
			Literal::string(&name_str.as_str()),
			func_ident,
			quote!(sweet::TestCaseConfig::Default),
		);
		let out = quote!(#func_out #submit).into();
		Ok(Self { out })
	}
}
