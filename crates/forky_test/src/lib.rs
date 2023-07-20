mod macros;
use macros::*;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn sweet_test(attr: TokenStream, input: TokenStream) -> TokenStream {
	TestCaseAttr::parse(attr, input)
}
#[proc_macro]
pub fn sweet(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as SuiteFunc).out
}

#[proc_macro]
pub fn test(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as TestCaseFunc).out
}
