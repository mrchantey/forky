use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_element(node: &NodeParser) -> TokenStream {
	let NodeParser {
		num_children,
		child_params,
		..
	} = node;

	let ident = crate::utils::parent_element(*num_children);
	let node_ident = crate::utils::parent_node(*num_children);

	let child_bounds = child_bounds(*num_children);
	let child_nodes = child_nodes(*num_children);
	let children_into_action = children_into_action(*num_children);
	let children_into_bundle = children_into_bundle(*num_children);
	let children_into_prop_bundle = children_into_prop_bundle(*num_children);
	let children_add_systems = children_add_systems(*num_children);
	let child_fields_def = child_fields_def(*num_children);
	let child_fields_args = child_fields_args(*num_children);
	let child_fields_assignment = child_fields_assignment(*num_children);
	let children_inferred_types = children_inferred_types(*num_children);
	let children_with_path = children_with_path(*num_children);
	let child_markers = child_markers(*num_children);

	let self_bounds = quote! {
		Path: TreePath,
		Action: IntoAction,
		Props: IntoPropBundle,
		#child_bounds
	};
	let self_params = quote! {
		Path,
		Action,
		Props,
		#child_params
	};

	quote! {

		#[derive(Clone)]
		pub struct #ident<#self_bounds>{
			action: Action,
			props: Props,
			phantom: std::marker::PhantomData<Path>,
			#child_fields_def
		}

		impl <#self_bounds> #ident<#self_params>{
			pub fn new<#child_markers>(action: Action, props: Props, #child_fields_args)->Self{
				Self{
					action,
					props,
					phantom: std::marker::PhantomData,
					#child_fields_assignment
				}
			}
		}

		impl <#self_bounds> AddSystems for #ident<#self_params>{
			fn add_systems(self, schedule: &mut Schedule) {
				Path::configure_sets(schedule);
				schedule.add_systems(self.action.action_into_system_configs::<<Self as TreeElement>::Node>());
				#children_add_systems
			}
		}

		impl<#self_bounds> IntoAction for #ident<#self_params>{
			fn action_into_system_configs<Node: AiNode>(self)-> SystemConfigs{
				(
					self.action.action_into_system_configs::<Node>(),
				#children_into_action
			).into_configs()
			}
		}
		impl<#self_bounds> IntoPropBundle for #ident<#self_params>{
			fn into_prop_bundle<Node: AiNode>(self)->impl Bundle{
				self.with_path::<Node>().into_bundle()
			}
		}
		impl<#self_bounds> IntoBundle for #ident<#self_params>{
			fn into_bundle(self)->impl Bundle{
				(
				self.props.into_prop_bundle::<<Self as TreeElement>::Node>(),
				#children_into_bundle
				)
			}
		}

		impl<#self_bounds> TreeElement for #ident<#self_params>{
			type Node = <#node_ident::<Path,#child_nodes> as AiNode>::WithPath<Path>;
			fn with_path<NewPath: TreePath>(self)->impl TreeElement{
				#ident::<NewPath, Action, Props, #children_inferred_types>::new(
					self.action,
					self.props,
					#children_with_path
				)
			}
		}
	}
}

fn children_into_action(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let name = child_field_name(index);
			quote!((self.#name.action_into_system_configs::<Node>(), #prev))
		})
		.into_token_stream()
}


fn children_into_bundle(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let name = child_field_name(index);
			quote!((self.#name.into_bundle(), #prev))
			// quote!((self.#name.into_prop_bundle::<Self>(), #prev))
		})
		.into_token_stream()
}
fn children_into_prop_bundle(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let name = child_field_name(index);
			quote!((self.#name.with_path::<TreePathSegment<#index, Node>>().into_prop_bundle::<Self>(), #prev))
		})
		.into_token_stream()
}
fn children_add_systems(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let name = child_field_name(index);
			quote!(self.#name.add_systems(schedule);)
		})
		.collect()
}

fn child_fields_args(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			let marker = child_marker_type(index);
			quote!(#field: impl IntoElement<#marker, Out=#ty>,)
		})
		.collect()
}
fn child_fields_assignment(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			quote!(#field:#field.into_element(),)
		})
		.collect()
}


fn child_bounds(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let ty = child_type_name(index);
			quote!(#ty:TreeElement,)
		})
		.collect()
}
fn child_nodes(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let ty = child_type_name(index);
			quote!(<#ty as TreeElement>::Node,)
		})
		.collect()
}
fn children_inferred_types(num_children: usize) -> TokenStream {
	(0..num_children).map(|_index| quote!(_,)).collect()
}

fn children_with_path(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let name = child_field_name(index);
			quote!(self.#name.with_path::<TreePathSegment<#index, NewPath>>(),)
		})
		.collect()
}
fn child_markers(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let ty = child_marker_type(index);
			quote!(#ty,)
		})
		.collect()
}
