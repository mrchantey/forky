use proc_macro2::Span;
use std::fmt::Display;
use syn::Ident;
use syn::ItemFn;




pub fn parent_node(index: usize) -> Ident {
	Ident::new(&format!("ParentNode{index}"), Span::call_site())
}

pub fn parent_action(index: usize) -> Ident {
	Ident::new(&format!("ParentAction{index}"), Span::call_site())
}
pub fn parent_element(index: usize) -> Ident {
	Ident::new(&format!("ParentElement{index}"), Span::call_site())
}
pub fn parent_prop_bundle(index: usize) -> Ident {
	Ident::new(&format!("ParentPropBundle{index}"), Span::call_site())
}
pub fn props_name(ident: impl Display) -> Ident {
	Ident::new(&format!("{ident}Props"), Span::call_site())
}

pub fn func_inner_ident(func: &ItemFn) -> Ident {
	Ident::new(&format!("{}_inner", func.sig.ident), func.sig.ident.span())
}

pub fn func_as_inner(func: &ItemFn) -> ItemFn {
	let mut func_inner = func.clone();
	func_inner.sig.ident = func_inner_ident(func);
	func_inner.vis = syn::Visibility::Inherited;
	func_inner
}
