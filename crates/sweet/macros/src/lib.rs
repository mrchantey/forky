mod macros;
use macros::*;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Mark a function to be ran by the sweet test runner.
///
/// # Accepted Signatures
/// ```rust
///
/// #[sweet_test]
/// fn empty() {}
///
/// #[sweet_test]
/// fn returns_result() -> sweet::Result<()> {}
///
/// #[sweet_test]
/// async fn is_async() {}
///
/// ```
///
///
/// # Attributes
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


/// Macro for declaring several tests with less boilerplate.
///
/// # Example
///
/// ```rust
///
/// sweet!{
///		it "has less boilerplate" {
///			expect(true).to_be_true()?;
///		}
///		test "is an alias for it"{}
///		it skip "wont run"{}
///		it only "will exclude non-onlys in this suite"{}
///		it e2e "(in-browser) runs in the parent process"{}
/// }
///
/// ```
#[proc_macro]
pub fn sweet(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as SuiteFunc).out
}

// #[proc_macro]
// pub fn test(input: TokenStream) -> TokenStream {
// 	parse_macro_input!(input as TestCaseFunc).out
// }
