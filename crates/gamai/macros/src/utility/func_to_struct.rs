use syn::Ident;
use syn::ItemFn;

pub fn func_inner_ident(ident: &Ident) -> Ident {
	Ident::new(&format!("{}_inner", ident), ident.span())
}

pub fn func_as_inner(func: &ItemFn) -> ItemFn {
	let mut func_inner = func.clone();
	func_inner.sig.ident = func_inner_ident(&func.sig.ident);
	func_inner
}
