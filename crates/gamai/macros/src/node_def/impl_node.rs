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
	let world_query = world_query_nested(node);
	let params = node_params_nested(node);
	let child_states = child_states(node);
	let node_recast = node_recast(node);
	let add_systems_children = add_systems_children(node);
	let configure_sets = configure_sets(node);

	quote!(
		impl<#self_bounds> AiNode for #ident<#self_params>
		{
			const NODE_ID: usize = NODE_ID;
			const GRAPH_ID: usize = GRAPH_ID;
			const GRAPH_DEPTH: usize = GRAPH_DEPTH;
			const CHILD_INDEX: usize = CHILD_INDEX;
			const PARENT_DEPTH: usize = PARENT_DEPTH;

			type ChildQuery = (
				Entity,
				// &'static mut DerefEdgeState<Self>,
				// Option<&'static mut DerefNodeState<Self>>,
				#world_query
			);

			fn add_systems(self, schedule: &mut Schedule){	
				self.node_system.into_node_system::<Self>(schedule, NodeSet::<GRAPH_ID, GRAPH_DEPTH>);
				self.edge_system.into_node_system::<Self>(schedule, NodeSet::<GRAPH_ID, GRAPH_DEPTH>);

				#configure_sets
				#add_systems_children
			}

			fn entity<'a>(val: &<Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>) ->Entity{
				val.0
			}

			fn children<'a>((entity,#params): <Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>)
				-> Vec<ChildState<'a>> {
					#node_recast
					vec![#child_states]
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

fn world_query_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let child = child_type_param_name(index);
			quote!((&'static mut DerefEdgeState<#child>,Option<&'static mut DerefNodeState<#child>>, #prev))
		})
		.into_token_stream()
}
fn node_params_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let edge = field_ident("edge", index);
			let node = field_ident("node", index);
			quote!((#edge, #node, #prev))
		})
		.into_token_stream()
}

fn child_states(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		.map(|index| {
			let child = child_type_param_name(index);
			let edge = field_ident("edge", index);
			let node = field_ident("node", index);
			quote! {
				ChildState::<'a>{
					entity: entity.clone(),
					index: #index,
					edge: #edge.into_inner(),
					node: #node,
					set_node_state_func: move |commands:&mut Commands,entity:Entity,state: NodeState|{
						commands.entity(entity).insert(DerefNodeState::<#child>::new(state));
					},
					remove_node_state_func: move |commands:&mut Commands,entity:Entity|{
						commands.entity(entity).remove::<DerefNodeState::<#child>>();
					},
				},
			}
		})
		.collect()
}

// there is probably a better way to do this
fn node_recast(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		.map(|index| {
			let node = field_ident("node", index);
			quote! {
				let #node = if let Some(val) = #node{
					Some(val.into_inner() as DerefNode<'_>)
				}else{
					None
				};
			}
		})
		.collect()
}

fn add_systems_children(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		.map(|index| {
			let child_ident = child_field_param_name(index);
			quote!(self.#child_ident.add_systems(schedule);)
		})
		.collect()
}
