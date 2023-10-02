use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_self(node: &NodeParser) -> TokenStream {
	// let states_typed = get_states_typed(node);
	let params_nested = child_params_nested(node);
	let NodeParser {
		ident,
		self_bounds,
		self_params_systems_only,
		num_edges,
		..
	} = node;

	let child_fields = child_fields(*num_edges);

	quote! {
		// #[derive(Debug,Default,Clone)]
		#[derive(Default,Clone)]
		pub struct #ident<#self_bounds>{
			phantom: std::marker::PhantomData<(#self_params_systems_only)>,
			children: (#params_nested),
			edge_state: DerefEdgeState<Self>,
			node_system: NodeSystem,
			edge_system: EdgeSystem,
			#child_fields
			// children: (#child_params),
		}
		// pub struct #ident<#self_bounds>(std::marker::PhantomData<(#self_params_systems_only)>);
	}
}

fn child_params_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = child_type_param_name(index);
			quote!((#ident, #prev))
		})
		.into_token_stream()
}
