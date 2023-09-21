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
	let world_query = all_edges_nested(node);
	let params = node_params_nested(node);
	let params_deref = node_params_deref(node);
	let set_child_node = impl_set_child_node(node);
	let add_systems_children = add_systems_children(node);
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

			type ChildQuery = (Entity, #world_query);

			fn set_child_node_state(commands: &mut Commands, entity: Entity, index: usize)-> gamai::Result<()> {
				match index {
					#set_child_node
					_ => gamai::bail!(format!("Node {}: child index {index} out of range", Self::NODE_ID)),
				}
			}
			fn add_systems(schedule: &mut Schedule){
				NodeSystem::add_node_system::<Self>(schedule, NodeSet::<GRAPH_ID, GRAPH_DEPTH>,&#node_system_config);
				//my edge should run before my parents node set
				EdgeSystem::add_node_system::<Self>(schedule, BeforeNodeSet::<GRAPH_ID, PARENT_DEPTH>, &NodeSystemConfig::default());

				#configure_sets
				#add_systems_children
			}
			fn plugin() -> impl Plugin{
				Self::default()
			}
			fn bundle() -> impl Bundle{
				Self::default()
			}
			fn entity<'a>(val: &<Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>) ->Entity{
				val.0
			}
			fn children<'a>((entity,#params): &<Self::ChildQuery as bevy_ecs::query::WorldQuery>::Item<'a>)
				-> Vec<&'a dyn std::ops::Deref<Target = EdgeState>>{
					vec![#params_deref]
			}

		}

		impl<#self_bounds> Plugin for #ident<#self_params> {
			fn build(&self, app: &mut bevy_app::App) {
				app.init_schedule(Update);
				let schedule = app.get_schedule_mut(Update).unwrap();
				<Self as AiNode>::add_systems(schedule);
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
			quote!(*#ident,)
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

fn add_systems_children(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		.map(|index| {
			let child_ident = child_type_param_name(index);
			quote!(#child_ident::add_systems(schedule);)
		})
		.collect()
}
