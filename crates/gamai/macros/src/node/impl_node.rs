use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_node(node: &NodeParser) -> TokenStream {
	let NodeParser { ident, .. } = node;
	// let AiNodeBuilder {
	// 	builder_ident: ident,
	// 	builder_bounds,
	// 	builder_params,
	// 	..
	// } = builder;
	let world_query = all_edges_nested(node);
	let params = node_params_nested(node);
	let params_deref = node_params_deref(node);
	let set_child_node = impl_set_child_node(node);

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
			fn set_child_node_state(commands: &mut Commands, entity: Entity, index: usize)-> gamai::Result<()> {
				match index {
					#set_child_node
					_ => gamai::bail!(Self::SET_CHILD_ERROR),
				};
				Ok(())
			}
		}
	)
}

fn all_edges_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = edge_type(node, index);
			quote!((&'static #ident, #prev))
		})
		.into_token_stream()
}
fn node_params_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = field_ident("edge", index);
			quote!((#ident, #prev))
		})
		.into_token_stream()
}

fn node_params_deref(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		.map(|index| {
			let ident = field_ident("edge", index);
			quote!(**#ident,)
		})
		.collect()
}

fn impl_set_child_node(node: &NodeParser) -> TokenStream {
	// let AiNode { ident, .. } = node;
	(0..node.num_edges)
		.map(|index| {
			let val = default_child_node_state(node, index);
			quote!(#index => commands.entity(entity).insert(#val),)
		})
		.collect()
}
