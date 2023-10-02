use super::*;
use quote::quote;

pub fn parse_node(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let num_edges = get_num_edges(tokens).unwrap();
	let node = NodeParser::new(num_edges);

	let impl_self = impl_self(&node);
	let impl_node = impl_node(&node);
	let impl_named_children = impl_named_children(&node);

	quote! {
		use bevy_app::prelude::*;
		use bevy_ecs::prelude::*;
		use gamai::*;
		#impl_self
		#impl_node
		#impl_named_children
	}
	.into()
}
