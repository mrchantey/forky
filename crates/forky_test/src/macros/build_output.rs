use super::*;
use proc_macro2::Group;
use proc_macro2::TokenStream;
use quote::quote;


pub fn parse_test_case(func: &Group, flags: &TestCaseFlags) -> TokenStream {
	let native = parse_test_case_native(func, flags);
	let wasm = parse_test_case_wasm(func, flags);
	quote!(
		use sweet::exports::*;
		#native
		#wasm
	)
}
