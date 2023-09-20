use super::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_named_children(node: &NodeParser) -> TokenStream {
	let NodeParser {
		ident,
		self_params,
		self_bounds,
		num_edges,
		..
	} = node;
	let trait_ident =
		Ident::new(&format!("NamedChildren{num_edges}"), node.ident.span());

	let child_types = child_types(*num_edges);
	let child_impl = child_impl(*num_edges);

	quote! {

		pub trait #trait_ident{
			#child_types
		}

		impl<#self_bounds> #trait_ident for #ident<#self_params>{
			#child_impl
		}
	}
}


fn child_types(num_edges: usize) -> TokenStream {
	(0..num_edges)
		.map(|index| {
			let child_ident = child_type_param_name(index);
			quote!(type #child_ident: AiNode;)
		})
		.collect()
}
fn child_impl(num_edges: usize) -> TokenStream {
	(0..num_edges)
		.map(|index| {
			let child_ident = child_type_param_name(index);
			quote!(type #child_ident = #child_ident;)
		})
		.collect()
}
