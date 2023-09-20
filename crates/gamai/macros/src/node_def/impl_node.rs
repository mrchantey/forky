use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_node(node: &NodeParser) -> TokenStream {
	let NodeParser {
		ident,
		self_params,
		self_bounds,
		..
	} = node;
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
	let build_children = build_children(node);
	let configure_sets = configure_sets(node);

	quote!(
		impl<#self_bounds> AiNode for #ident<#self_params>
		{
			type NodeSystem = NodeSystem;
			type EdgeSystem = EdgeSystem;
			// type Parent = Parent;

			const NODE_ID: usize = NODE_ID;
			const GRAPH_ID: usize = GRAPH_ID;
			const GRAPH_DEPTH: usize = GRAPH_DEPTH;
			const CHILD_INDEX: usize = CHILD_INDEX;
			const PARENT_DEPTH: usize = PARENT_DEPTH;

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
					_ => gamai::bail!(format!("Node {}: child index {index} out of range", Self::NODE_ID)),
				}
			}
			fn build(schedule: &mut Schedule){
				NodeSystem::add_node_system::<Self>(schedule, Self::set_update());

				#configure_sets
				#build_children
			}

			fn set_pre_update() -> impl SystemSet { NodePreUpdate::<GRAPH_ID, GRAPH_DEPTH> }
			fn set_update() -> impl SystemSet { NodeUpdate::<GRAPH_ID, GRAPH_DEPTH> }
			fn set_post_update() -> impl SystemSet {NodePostUpdate::<GRAPH_ID, GRAPH_DEPTH>}
		}
	)
}

fn configure_sets(node: &NodeParser) -> TokenStream {
	// repeats set configuration for each siblings, thats ok

	let common = quote!(
		let is_root = GRAPH_DEPTH == 0 && PARENT_DEPTH == 0;
		if !is_root{
			schedule.configure_set(Self::set_post_update().before(NodePostUpdate::<GRAPH_ID, PARENT_DEPTH>));
		}
		schedule.configure_set(Self::set_update().before(Self::set_post_update()));
		schedule.configure_set(Self::set_pre_update().before(Self::set_update()));
	);
	if node.num_edges == 0 {
		common
	} else {
		// let child_ident = child_type_param_name(0);
		quote! {
			#common
		}
	}
}

fn all_edges_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let child = child_type_param_name(index);
			quote!((&'static ChildEdgeState<#child>, #prev))
		})
		.into_token_stream()
}
fn node_params_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = field_ident("child", index);
			quote!((#ident, #prev))
		})
		.into_token_stream()
}

fn node_params_deref(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		.map(|index| {
			let ident = field_ident("child", index);
			quote!(**#ident,)
		})
		.collect()
}

fn impl_set_child_node(node: &NodeParser) -> TokenStream {
	// let AiNode { ident, .. } = node;
	(0..node.num_edges)
		.map(|index| {
			let child = child_type_param_name(index);
			quote!(#index => {
				commands.entity(entity).insert(ChildNodeState::<#child>::default());
				Ok(())
			},)
		})
		.collect()
}

fn build_children(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		.map(|index| {
			let child_ident = child_type_param_name(index);
			quote!(#child_ident::build(schedule);)
		})
		.collect()
}
