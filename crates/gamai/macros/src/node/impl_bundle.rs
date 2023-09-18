use super::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_bundle(node: &NodeParser) -> TokenStream {
	let NodeParser { ident, vis, .. } = node;
	let ident = Ident::new(&format!("{ident}Bundle"), ident.span());
	let choice_states = choice_states(node);
	quote! {
		#[derive(Bundle, Default)]
		#vis struct #ident{
			#choice_states
		}
	}
}
fn choice_states(node: &NodeParser) -> TokenStream {
	(0..node.num_choices)
		.map(|index| {
			let edge_field = field_ident("edge", index);
			let edge_type = edge_type(node, index);
			// let node_field = field_ident("node", index);
			// let node_type = node_type(node, index);
			quote!(
				#edge_field: #edge_type,
				// #node_field: #node_type,
			)
		})
		.collect()
}
