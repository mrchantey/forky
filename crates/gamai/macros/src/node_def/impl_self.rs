use super::*;
use proc_macro2::TokenStream;
use quote::quote;
// use quote::ToTokens;

pub fn impl_self(node: &NodeParser) -> TokenStream {
	// let states_typed = get_states_typed(node);
	// let params_nested = child_params_nested(node);
	let NodeParser {
		ident,
		self_bounds,
		self_params,
		self_params_systems_only,
		num_edges,
		..
	} = node;

	let child_fields_def = child_fields_def(*num_edges);
	let child_fields = child_fields(*num_edges);

	quote! {
		#[derive(Clone)]
		pub struct #ident<#self_bounds>{
			phantom: std::marker::PhantomData<(#self_params_systems_only)>,
			node_system: NodeSystem,
			edge_system: EdgeSystem,
			#child_fields_def
		}

		impl<#self_bounds> #ident<#self_params> {
			pub fn new(node_system: NodeSystem, edge_system: EdgeSystem, #child_fields_def) -> Self {
				Self {
					phantom: PhantomData,
					node_system,
					edge_system,
					#child_fields
				}
			}
		}
	}
}
