use proc_macro2::Span;
use syn::Ident;




pub fn parent_node(index: usize) -> Ident {
	Ident::new(&format!("Node{index}"), Span::call_site())
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
