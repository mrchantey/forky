use super::*;
use proc_macro2::Ident;
use proc_macro2::Span;
// use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
// use quote::ToTokens;

pub fn impl_action(node: &NodeParser) -> TokenStream {
	let NodeParser {
		// ident,
		// child_bounds,
		num_children,
		child_params,
		// self_params,
		// self_bounds,
		..
	} = node;

	let child_bounds = child_bounds(*num_children);
	let children_add_systems = children_add_systems(*num_children);
	let child_fields_def = child_fields_def(*num_children);
	let ident =
		Ident::new(&format!("ParentAction{num_children}"), Span::call_site());

	let self_bounds = quote!(Action: AddSystems, #child_bounds);
	let self_params = quote!(Action, #child_params);

	quote! {

		pub struct #ident<#self_bounds>{
			action:Action,
			#child_fields_def
		}

		impl <#self_bounds>
			AddSystems for #ident<#self_params>{
			fn add_systems(self, schedule: &mut Schedule) {
				self.action.add_systems(schedule);
				#children_add_systems
			}
		}

	}
}
fn children_add_systems(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let name = child_field_name(index);
			quote!(self.#name.add_systems(schedule);)
		})
		.collect()
}

fn child_bounds(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let ty = child_type_name(index);
			quote!(#ty:AddSystems,)
		})
		.collect()
}
// fn get_children(num_children: usize) -> TokenStream {
// 	(0..num_children)
// 		.map(|index| {
// 			let child_ident = child_field_name(index);
// 			quote!(&self.#child_ident,)
// 		})
// 		.collect()
// }
