use super::*;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;

pub struct TestCaseFunc {
	pub out: proc_macro::TokenStream,
}

impl Parse for TestCaseFunc {
	fn parse(stream: ParseStream) -> Result<Self> {
		let mut iter = into_peekable(stream)?;
		let config = parse_case_config(&mut iter)?;
		let name = parse_name(&mut iter);
		try_remove_comma(&mut iter);
		try_remove_comma(&mut iter);

		let mut func = TokenStream::new();
		while let Some(t) = iter.next() {
			func.append(t);
		}

		let out = to_inventory_wrap_func(name, func, config).into();

		Ok(Self { out })
	}
}
