// use super::*;
use super::parse_test_case;
use super::TestCaseFlags;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;
use syn::ReturnType;

pub struct TestCaseAttr {
	// pub out: TokenStream,
}

impl TestCaseAttr {
	pub fn parse(
		attr: proc_macro::TokenStream,
		input: proc_macro::TokenStream,
	) -> syn::Result<TokenStream> {
		let func = syn::parse::<ItemFn>(input)?;

		// panic!("Not implemented, maybe never will");
		let func_out = func.clone();
		let ident = func.sig.ident;
		let attr: TokenStream = attr.into();
		let mut attr = attr.into_iter().peekable();

		let mut flags = TestCaseFlags::parse(&mut attr)?;
		let name = ident.to_string().replace("_", " ");
		flags.name = Literal::string(&name);

		let is_async = func.sig.asyncness.is_some();
		let is_result = func.sig.output != ReturnType::Default;

		let wrapped = match (is_async, is_result) {
			(true, true) => quote! {#ident().await?;},
			(true, false) => quote! {#ident().await;},
			(false, true) => quote! {#ident()?;},
			(false, false) => quote! {#ident();},
		};

		let submit = parse_test_case(&wrapped, &flags);
		let out = quote! {
			#func_out
			#submit
		}
		.into();
		Ok(out)
	}
}
