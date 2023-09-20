use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_node(node: &NodeParser) -> TokenStream {
	let NodeParser {
		ident,
		self_params,
		self_bounds,
		num_edges,
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

	let node_system_config = if *num_edges == 0 {
		quote! {NodeSystemConfig::default()}
	} else {
		quote! {NodeSystemConfig{apply_deferred: true,}}
	};

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
				NodeSystem::add_node_system::<Self>(schedule, NodeSet::<GRAPH_ID, GRAPH_DEPTH>,&#node_system_config);
				//my edge should run before my parents node set
				EdgeSystem::add_node_system::<Self>(schedule, BeforeNodeSet::<GRAPH_ID, PARENT_DEPTH>, &NodeSystemConfig::default());

				#configure_sets
				#build_children
			}

		}
	)
}

fn configure_sets(_node: &NodeParser) -> TokenStream {
	// repeats set configuration for each siblings, thats ok
	quote!(
		if GRAPH_DEPTH != 0{
			schedule.configure_set(BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>
				.after(NodeSet::<GRAPH_ID, PARENT_DEPTH>));
		}
		schedule.configure_set(NodeSet::<GRAPH_ID, GRAPH_DEPTH>
			.after(BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>));
	)
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
