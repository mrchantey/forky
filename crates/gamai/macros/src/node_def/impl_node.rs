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
	let get_children = get_children(*num_children);
	let match_get_children = match_get_children(*num_children);
	let match_get_children_owned = match_get_children_owned(*num_children);

	let children_inferred_types = children_inferred_types(*num_children);
	let children_into_child = children_into_child(*num_children);

	quote! {
		impl<#self_bounds> TreePath for #ident<#self_params> {
			type Parent = Path::Parent;
			const CHILD_INDEX: usize = Path::CHILD_INDEX;
		}

		// impl<#self_bounds> IntoNode for #ident<#self_params> {
		// 	type Out = Self;
		// 	fn into_node(self) -> Self::Out { self }
		// }

		impl<#self_bounds> AiNode for #ident<#self_params> {

			type ChildQuery = (
				Entity,
				#world_query
			);

			#[allow(unused_parens)]
			type ChildBundle = (#child_bundles);

			fn add_systems(self, schedule: &mut Schedule){
				Self::configure_sets(schedule);
				schedule.add_systems(self.system.into_node_system_configs::<Self>());

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
			fn get_children(&self)->Vec<&dyn NodeInspector>{
				vec![#get_children]
			}
			fn into_child<NewPath: TreePath>(self) -> impl AiNode {
				#ident::<NewPath, _, #children_inferred_types>::new(
					self.system,
					#children_into_child
				)
			}
		}
	}
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
fn get_children(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_field_name(index);
			quote!(&self.#child_ident,)
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
fn children_inferred_types(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|_index| {
			quote!(_,)
		})
		.collect()
}
fn children_into_child(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_field = child_field_name(index);
			quote!(self.#child_field.into_child::<TreePathSegment<#index, NewPath>>(),)
		})
		.collect()
}
