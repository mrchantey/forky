use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;



// currently very basic, maybe allow children etc in the future
pub fn parse_tree_builder(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let func = parse_macro_input!(item as ItemFn);

	if is_snake_case(func.sig.ident.to_string().as_str()) {
		return syn::Error::new(
			func.sig.ident.span(),
			"tree builder function must be PascalCase",
		)
		.into_compile_error()
		.into();
	}
	quote! {
		#[allow(non_snake_case)]
		#func
	}
	.into()
}


fn is_snake_case(val: &str) -> bool {
	val.chars().next().unwrap().is_lowercase()
}
