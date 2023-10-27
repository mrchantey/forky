use super::*;
use proc_macro2::Ident;
use proc_macro2::Span;
// use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
// use quote::ToTokens;

pub fn impl_element(node: &NodeParser) -> TokenStream {
	let NodeParser {
		num_children,
		child_params,
		..
	} = node;

	let child_bounds = child_bounds(*num_children);
	let children_into_bundle = children_into_bundle(*num_children);
	let children_add_systems = children_add_systems(*num_children);
	let child_fields_def = child_fields_def(*num_children);
	let child_fields_args = child_fields_args(*num_children);
	let child_fields_assignment = child_fields_assignment(*num_children);
	let ident =
		Ident::new(&format!("ParentElement{num_children}"), Span::call_site());

	let self_bounds = quote! {
		Node: AiNode,
		Action: IntoAction,
		Props: IntoPropBundle,
		#child_bounds
	};
	let self_params = quote! {
		Node,
		Action,
		Props,
		#child_params
	};

	quote! {

		pub struct #ident<#self_bounds>{
			node: Node,
			action: Action,
			props: Props,
			#child_fields_def
		}

		impl <#self_bounds> #ident<#self_params>{
			pub fn new(node: Node, action: Action, props: Props,#child_fields_args)->Self{
				Self{
					node,
					action,
					props,
					#child_fields_assignment
				}
			}
		}

		impl <#self_bounds> AddSystems for #ident<#self_params>{
			fn add_systems(self, schedule: &mut Schedule) {
				schedule.add_systems(self.action.into_action_configs::<Node>());
				#children_add_systems
			}
		}

		impl<#self_bounds> IntoBundle for #ident<#self_params>{
			fn into_bundle(self)->impl Bundle{
				(
				// self.props.into_bundle::<Node>(),
				#children_into_bundle
				)
			}
		}


	}
}

fn children_into_bundle(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let name = child_field_name(index);
			quote!((self.#name.into_bundle(), #prev))
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
			quote!(#field: #ty,)
		})
		.collect()
}
fn child_fields_assignment(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			quote!(#field:#field,)
		})
		.collect()
}


fn child_bounds(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let ty = child_type_name(index);
			quote!(#ty:AddSystems + IntoBundle,)
		})
		.collect()
}
