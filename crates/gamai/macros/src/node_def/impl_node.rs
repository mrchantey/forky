use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_node(node: &NodeParser) -> TokenStream {
	let NodeParser {
		ident,
		num_children,
		self_params,
		self_bounds,
		..
	} = node;
	let world_query = world_query_nested(node);
	let node_params = node_params_nested(node);
	let child_bundles = child_bundles_nested(node);
	let child_states = child_states(node);
	let node_recast = node_recast(node);
	let add_systems_children = add_systems_children(*num_children);
	let configure_sets = configure_sets(node);
	let match_get_children = match_get_children(*num_children);
	let match_get_children_owned = match_get_children_owned(*num_children);

	quote! {
			impl<#self_bounds> IntoNodeId for #ident<#self_params>{
				const GRAPH_ID: usize = GRAPH_ID;
				const GRAPH_DEPTH: usize = GRAPH_DEPTH;
				const CHILD_INDEX: usize = CHILD_INDEX;
				const NODE_ID: usize = NODE_ID;
				const PARENT_DEPTH: usize = GRAPH_DEPTH;
			}

			impl<#self_bounds> AiNode for #ident<#self_params>
			{

				type ChildQuery = (
					Entity,
					// &'static mut DerefEdgeState<Self>,
					// Option<&'static mut DerefNodeState<Self>>,
					#world_query
				);

				#[allow(unused_parens)]
				type ChildBundle = (#child_bundles);

				fn add_systems(self, schedule: &mut Schedule){
					//TODO handle attributes
					// self.node_system.into_node_system::<Self>(schedule, NodeSet::<GRAPH_ID, GRAPH_DEPTH>);
					//edges run before parent
					// self.edge_system.into_node_system::<Self>(schedule, BeforeNodeSet::<GRAPH_ID, PARENT_DEPTH>);

					#configure_sets
					#add_systems_children
				}

				fn entity<'a>(val: &<Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>) ->Entity{
					val.0
				}

				fn children<'a>((entity,#node_params): <Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>)
					-> Vec<ChildState<'a>> {
						#node_recast
						vec![#child_states]
				}

				fn get_child(&self,index:usize)->&dyn NodeInspector{
					match index{
						#match_get_children
						_=> panic!("invalid child index")
					}
				}
				fn get_child_owned(self,index:usize)->Box<dyn NodeInspector>{
					match index{
						#match_get_children_owned
						_=> panic!("invalid child index")
					}
				}
			}

			impl<#self_bounds> IntoRootNode for #ident<#self_params>{
				type Out = #ident<#self_params>;
				fn into_root_node(self) -> Self::Out{
					self
				}
			}
	}
}
// TODO move outside of macro when generic_const_expr stabilizes
fn configure_sets(_node: &NodeParser) -> TokenStream {
	// repeats set configuration for each siblings, thats ok
	quote!(if GRAPH_DEPTH != 0 {
		// run my before_node_set after parent node_set
		schedule.configure_set(
			BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>
				.after(NodeSet::<GRAPH_ID, PARENT_DEPTH>),
		);
		// run my node_set after my before_node_set
		schedule.configure_set(
			NodeSet::<GRAPH_ID, GRAPH_DEPTH>
				.after(BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>),
		);
	} else {
		// if root run my before_node_set before my node_set
		schedule.configure_set(
			BeforeNodeSet::<GRAPH_ID, GRAPH_DEPTH>
				.before(NodeSet::<GRAPH_ID, GRAPH_DEPTH>),
		);
	})
}

fn world_query_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_children)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let child = child_type_name(index);
			quote!((&'static mut DerefEdgeState<#child>,Option<&'static mut DerefNodeState<#child>>, #prev))
		})
		.into_token_stream()
}
fn node_params_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_children)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let edge = field_ident("edge", index);
			let node = field_ident("node", index);
			quote!((#edge, #node, #prev))
		})
		.into_token_stream()
}

fn child_states(node: &NodeParser) -> TokenStream {
	(0..node.num_children)
		.map(|index| {
			let child = child_type_name(index);
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

/// returns (AiBundle<Child0>,(AiBundle<Child1>,..))
fn child_bundles_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_children)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = child_type_name(index);
			quote!((AiBundle<#ident>, #prev))
		})
		.into_token_stream()
}

// there is probably a better way to do this
fn node_recast(node: &NodeParser) -> TokenStream {
	(0..node.num_children)
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

fn add_systems_children(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_field_name(index);
			quote!(self.#child_ident.add_systems(schedule);)
		})
		.collect()
}
fn match_get_children(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_field_name(index);
			quote!(#index => &self.#child_ident,)
		})
		.collect()
}
fn match_get_children_owned(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_field_name(index);
			quote!(#index => Box::new(self.#child_ident),)
		})
		.collect()
}
