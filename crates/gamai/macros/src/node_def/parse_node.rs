use super::*;
use quote::quote;

pub fn parse_node(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let num_children = get_num_children(tokens).unwrap();
	let node = NodeParser::new(num_children);

	let impl_node = impl_node(&node);
	let impl_action = impl_action(&node);
	let impl_prop_bundle = prop_bundle(&node);
	let impl_element = impl_element(&node);

	quote! {
		use gamai::*;
		use gamai::exports::*;
		use gamai::node::*;
		use gamai::prop::*;
		use gamai::tree::*;
		#impl_node
		#impl_action
		#impl_prop_bundle
		#impl_element
	}
	.into()
}
