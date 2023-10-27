use super::*;
use crate::utils::parent_prop_bundle;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn prop_bundle(node: &NodeParser) -> TokenStream {
	let NodeParser {
		num_children,
		child_params,
		..
	} = node;

	let child_bounds = child_bounds(*num_children);
	let children_into_bundle = children_into_bundle(*num_children);
	let child_fields_def = child_fields_def(*num_children);
	let child_fields_args = child_fields_args(*num_children);
	let child_fields_assignment = child_fields_assignment(*num_children);
	let ident = parent_prop_bundle(*num_children);
	let self_bounds = quote!(Props: IntoBundle, #child_bounds);
	let self_params = quote!(Props, #child_params);

	quote! {

		pub struct #ident<#self_bounds>{
			props:Props,
			#child_fields_def
		}

		impl <#self_bounds> #ident<#self_params>{
			pub fn new(props: Props, #child_fields_args)->Self{
				Self{
					props,
					#child_fields_assignment
				}
			}
		}

		impl<#self_bounds> IntoBundle for #ident<#self_params>{
			fn into_bundle(self)->impl Bundle{
				(
				self.props.into_bundle(),
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
			quote!(#ty:IntoBundle,)
		})
		.collect()
}
