// use crate::*;
// use proc_macro2::TokenStream;
// use quote::quote;
// use std::sync::atomic::AtomicUsize;
// use std::sync::atomic::Ordering;
// use syn::parse_macro_input;
// use syn::parse_quote;
// use syn::token::Gt;
// use syn::token::Lt;
// use syn::ItemFn;

// pub fn parse_tree_builder(
// 	_attr: proc_macro::TokenStream,
// 	item: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
// 	let func = parse_macro_input!(item as ItemFn);
// 	// let node = NodeParser::new(item, 0);
// 	let ItemFn { vis, sig, .. } = func.clone();
// 	let ident = &sig.ident;

// 	let inner_func = inner_func(&func);
// 	let impl_into_tree = impl_into_tree(&func);
// 	quote! {
// 		use bevy_ecs::prelude::*;
// 		use gamai::*;

// 		#inner_func

// 		#[allow(non_camel_case_types)]
// 		#vis struct #ident;
// 		#impl_into_tree
// 	}
// 	.into()
// }

// #[rustfmt::skip]
// fn inner_func(func: &ItemFn) -> ItemFn {
// 	let mut func = func_as_inner(func);
// 	func.sig.generics.lt_token = Some(Lt::default());
// 	func.sig.generics.gt_token = Some(Gt::default());
// 	func.sig.generics.params.push(parse_quote!{const CHILD_INDEX: usize});
// 	func.sig.generics.params.push(parse_quote!{const GRAPH_ID: usize});
// 	func.sig.generics.params.push(parse_quote!{const PARENT_DEPTH: usize});
// 	func.sig.generics.params.push(parse_quote!{const GRANDPARENT_DEPTH: usize});
// 	func.sig.generics.params.push(parse_quote!{Parent});
// 	func.sig.generics.make_where_clause().predicates.push(parse_quote!{
// 		Parent: IntoNodeId<
// 			GRAPH_ID = { GRAPH_ID },
// 			GRAPH_DEPTH = { PARENT_DEPTH },
// 			PARENT_DEPTH = { GRANDPARENT_DEPTH },
// 		>
// 	});
// 	func
// }
