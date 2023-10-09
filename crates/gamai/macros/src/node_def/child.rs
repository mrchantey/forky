use super::*;
use proc_macro2::TokenStream;
use quote::ToTokens;

/// returns childN
pub fn child_field_name(index: usize) -> TokenStream {
	field_ident("child", index).to_token_stream()
}

/// Returns ChildN
pub fn child_type_name(index: usize) -> TokenStream {
	field_ident("Child", index).to_token_stream()
}
pub fn child_type_name_new(index: usize) -> TokenStream {
	field_ident("NewChild", index).to_token_stream()
}
