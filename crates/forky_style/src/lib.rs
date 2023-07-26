mod macros;
use macros::*;
use proc_macro::TokenStream;

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
	let stylesheet = get_stylesheet(input);
	let classes = get_classes(&stylesheet);
	classes_to_tokens(classes).into()
}
