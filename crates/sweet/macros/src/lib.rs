mod macros;
use macros::*;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Test Case Attribute
/// Will successfully parse the following signatures:
/// ```rs
/// fn empty() {}
/// fn returns_result() -> Result<()> {}
/// async fn is_async() {}
///
/// ```
///
///
/// Accepts several markers:
/// - `#[sweet_test(skip)]`: Skips the test
/// - `#[sweet_test(only)]`: Skips all other tests in file
/// - `#[sweet_test(e2e)]`: Runs in-browser wasm tests in a seperate process as an iframe
/// - `#[sweet_test(non_send)]`: Always runs the test in the main thread which is required in crates like `bevy` and `fantoccini`.
///
#[proc_macro_attribute]
pub fn sweet_test(attr: TokenStream, input: TokenStream) -> TokenStream {
	TestCaseAttr::parse(attr, input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}
#[proc_macro]
pub fn sweet(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as SuiteFunc).out
}

// #[proc_macro]
// pub fn test(input: TokenStream) -> TokenStream {
// 	parse_macro_input!(input as TestCaseFunc).out
// }
