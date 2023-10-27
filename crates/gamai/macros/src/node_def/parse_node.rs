use super::*;
use quote::quote;

pub fn parse_node(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let num_children = get_num_children(tokens).unwrap();
	let node = NodeParser::new(num_children);

	let impl_self = impl_self(&node);
	let impl_action = impl_action(&node);
	let impl_node = impl_node(&node);

	quote! {
		use gamai::*;
		use gamai::exports::*;
		use gamai::node::*;
		use gamai::prop::*;
		#impl_self
		#impl_node
		#impl_action
	}
	.into()
}
