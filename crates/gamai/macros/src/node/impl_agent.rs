use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_node(node: &AiNode) -> TokenStream {
	let AiNode { ident, .. } = node;
	// let AiNodeBuilder {
	// 	builder_ident: ident,
	// 	builder_bounds,
	// 	builder_params,
	// 	..
	// } = builder;
	let world_query = all_edges_nested(node);
	let params = node_params_nested(node);
	let params_deref = node_params_deref(node);
	let set_child_node = set_child_node_state(node);

	quote!(
		impl AiNode for #ident
		{
			type ChildrenQuery = (Entity, #world_query);
			fn edges(query: &Query<Self::ChildrenQuery>) -> Vec<(Entity, Vec<EdgeState>)> {
				query
					.iter()
					.map(|(entity, #params)| (entity, vec![#params_deref]))
					.collect()
			}
			fn set_child_node_state(commands: &mut Commands, entity: Entity, index: usize) {
				match index {
					#set_child_node
					_ => panic!("index out of range"),
				};
			}
		}
	)
}

fn all_edges_nested(node: &AiNode) -> TokenStream {
	(0..node.num_choices)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = edge_type(node, index);
			quote!((&'static #ident, #prev))
		})
		.into_token_stream()
}
fn node_params_nested(node: &AiNode) -> TokenStream {
	(0..node.num_choices)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = field_ident("edge", index);
			quote!((#ident, #prev))
		})
		.into_token_stream()
}

fn node_params_deref(node: &AiNode) -> TokenStream {
	(0..node.num_choices)
		.map(|index| {
			let ident = field_ident("edge", index);
			quote!(**#ident,)
		})
		.collect()
}

fn set_child_node_state(node: &AiNode) -> TokenStream {
	// let AiNode { ident, .. } = node;
	(0..node.num_choices)
		.map(|index| {
			let val = default_child_node_state(node, index);
			quote!(#index => commands.entity(entity).insert(#val),)
		})
		.collect()
}
