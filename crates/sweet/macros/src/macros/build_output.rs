use super::*;
use proc_macro2::TokenStream;
use quote::quote;


pub fn parse_test_case(func: &TokenStream, flags: &TestCaseFlags) -> TokenStream {
	let native = parse_test_case_native(func, flags);
	let wasm = parse_test_case_wasm(func, flags);
	quote!(
		// use sweet::*; //try not to pollute macro?
		use sweet::exports::*;
		#native
		#wasm
	)
}