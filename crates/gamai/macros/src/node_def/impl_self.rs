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
		self_bounds_full,
		self_params,
		phantom_types,
		num_edges,
		..
	} = node;

	let child_fields_def = child_fields_def(*num_edges);
	let child_fields_args = child_fields_args(*num_edges);
	let child_fields = child_fields_into_node(*num_edges);

	quote! {
		// #[derive(Clone)]
		pub struct #ident<#self_bounds>{
			phantom: std::marker::PhantomData<(#phantom_types)>,
			node_system: NodeSystem,
			edge_system: EdgeSystem,
			#child_fields_def
		}

		impl<#self_bounds_full> #ident<#self_params> {
			pub fn new(node_system: NodeSystem, edge_system: EdgeSystem, #child_fields_args) -> Self {
				Self {
					phantom: std::marker::PhantomData,
					node_system,
					edge_system,
					#child_fields
				}
			}
			// pub fn to_child_node<
			// 	const CHILD_NODE_ID: usize,
			// 	const CHILD_GRAPH_ID: usize,
			// 	const CHILD_GRAPH_DEPTH: usize,
			// 	const CHILD_CHILD_INDEX: usize,
			// 	const CHILD_PARENT_DEPTH:usize
			// 	>(self)->Self<#to_child_type_params>{
			// 	Self<#to_child_type_params>::new(self.node_system, self.edge_system, #child_fields)//todo self.child_field
			// }
		}
	}
}

fn child_fields_def(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			quote!(#field: #ty,)
		})
		.collect()
}

fn child_fields_into_node(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			quote!(#field: #field.into_child_node(),)
			// quote!(#field: #field.into_node::<#index,Self>(),)
		})
		.collect()
}
fn child_fields_args(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			// let ty = into_child_type_name(index);
			// quote!(#field: impl IntoNode<#index,Self,Out=#ty>,)
			quote!{#field: impl IntoChildNode<#index,
				PhantomNodeId<
					GRAPH_ID,
					GRAPH_DEPTH,
					CHILD_INDEX,
					PARENT_DEPTH
				>,Out=#ty>,
			}
			// quote!(#field: impl IntoNode<#index,PhantomNodeId<0,0,0,0>,Out=#ty>,)
			// quote!(#field: #ty,)
		})
		.collect()
}