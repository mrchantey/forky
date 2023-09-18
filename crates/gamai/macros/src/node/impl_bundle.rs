use super::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_bundle(node: &AiNode) -> TokenStream {
	let AiNode { ident, vis, .. } = node;
	let ident = Ident::new(&format!("{ident}Bundle"), ident.span());
	let choice_states = choice_states(node);
	quote! {
		#[derive(Bundle, Default)]
		#vis struct #ident{
			#choice_states
		}
	}
}
fn choice_states(node: &AiNode) -> TokenStream {
	(0..node.num_choices)
		.map(|index| {
			let edge_field = field_ident("edge", index);
			let edge_type = edge_type(node, index);
			// let action_field = field_ident("action", index);
			// let action_type = action_type(node, index);
			quote!(
				#edge_field: #edge_type,
				// #action_field: #action_type,
			)
		})
		.collect()
}
