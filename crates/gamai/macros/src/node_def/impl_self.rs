use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_self(node: &NodeParser) -> TokenStream {
	let NodeParser {
		ident,
		self_bounds,
		self_params,
		num_children,
		..
	} = node;

	let child_fields_def = child_fields_def(*num_children);
	let child_fields_args = child_fields_args(*num_children);
	let child_fields = child_fields_assignment(*num_children);
	let child_fields_markers = child_fields_markers(*num_children);

	quote! {
		#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
		pub struct #ident<#self_bounds>{
			phantom: std::marker::PhantomData<Path>,
			#child_fields_def
		}

		impl<#self_bounds> #ident<#self_params> {
			pub fn new<#child_fields_markers>(#child_fields_args) -> Self {
				Self {
					phantom: std::marker::PhantomData,
					#child_fields
				}
			}
		}
	}
}


fn child_fields_args(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			let marker = child_marker_name(index);
			quote!(#field: impl IntoNode<#marker, Out=#ty>,)
		})
		.collect()
}

fn child_fields_assignment(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			quote!(#field:#field.into_node(),)
		})
		.collect()
}


pub fn child_marker_name(index: usize) -> TokenStream {
	field_ident("IntoChildMarker", index).to_token_stream()
}


fn child_fields_markers(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_marker_name(index);
			quote!(#field,)
		})
		.collect()
}
