use proc_macro2::Ident;
use proc_macro2::Span;

pub fn _type_ident(struct_ident: &Ident, suffix: &str, num: usize) -> Ident {
	Ident::new(&format!("{struct_ident}{suffix}{num}"), Span::call_site())
}
pub fn field_ident(suffix: &str, num: usize) -> Ident {
	Ident::new(&format!("{suffix}{num}"), Span::call_site())
}
