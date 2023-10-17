use super::*;
use quote::quote;

pub fn parse_node(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let num_children = get_num_children(tokens).unwrap();
	let node = NodeParser::new(num_children);

	let impl_self = impl_self(&node);
	let impl_node = impl_node(&node);
	// let into_child_node = into_child_node(&node);
	// let impl_named_children = impl_named_children(&node);

	quote! {
		use gamai::*;
		use gamai::exports::*;
		use gamai::node::*;
		#impl_self
		#impl_node
		// #into_child_node
		// #impl_named_children
	}
	.into()
}
