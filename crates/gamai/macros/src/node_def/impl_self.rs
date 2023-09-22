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
		..
	} = node;

	quote! {
		// #[derive(Debug,Default,Clone)]
		#[derive(Debug,Default,Clone,Bundle)]
		// #[allow(non_camel_case_types)]
		pub struct #ident<#self_bounds>{
			phantom: PhantomComponent<NODE_ID,(#self_params_systems_only)>,
			// phantom: std::marker::PhantomData<(#self_params_systems_only)>,
			children: (#params_nested),
			edge_state: DerefEdgeState<Self>,
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
