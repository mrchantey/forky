use crate::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

const GENERIC_ERROR:&str = "a `node_system` must have a single type parameter bound by `gamai::AiNode` ie: \npub fn my_func<Node: AiNode>()`";


pub fn impl_into_node_system(func: &ItemFn) -> TokenStream {
	let ident = &func.sig.ident;
	let func_ident = func_ident(&func.sig.ident);

	let generic_err = assert_single_generic_bound(
		func.sig.generics.clone(),
		"AiNode",
		GENERIC_ERROR,
	)
	.unwrap_or_else(syn::Error::into_compile_error);

	quote! {
		// impl<M> NodeSystemBuilder<M> for #ident
		// {
		// 	fn get_system<Node: AiNode>(self) -> impl IntoSystemConfigs<M> { #func_ident::<Node> }
		// }

		// impl IntoNodeSystem<Self> for #ident 
		// // impl IntoNodeSystem<Self> for fn() -> #ident 
		// // impl<F> IntoNodeSystem<Self> for F 
		// // where F: 'static + Send + Sync + Fn()-> #ident
		// {
		// 	fn into_node_system<Node: AiNode>(
		// 		self,
		// 		schedule: &mut Schedule,
		// 		set: impl SystemSet,
		// 	) {
		// 		schedule.add_systems(#func_ident::<Node>.in_set(set));
		// 	}
		// }
		#generic_err
	}
}

pub fn func_ident(ident: &Ident) -> syn::Ident {
	syn::Ident::new(&format!("{}_func", ident), ident.span())
}

pub fn parse_original_function(func: &ItemFn) -> ItemFn {
	let mut func_inner = func.clone();
	func_inner.sig.ident = func_ident(&func.sig.ident);
	func_inner
}
