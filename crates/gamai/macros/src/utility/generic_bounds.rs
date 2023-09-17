use proc_macro2::Span;
use proc_macro2::TokenStream;
use syn::Generics;

pub fn assert_single_generic_bound(
	generics: Generics,
	expected_bound:&str,
	err:&str,
) -> Result<TokenStream, syn::parse::Error> {
	if generics.params.len() == 1 {
		let param = generics.params.first().unwrap();
		if let syn::GenericParam::Type(param) = param {
			if param.bounds.len() == 1 {
				if let syn::TypeParamBound::Trait(bound) =
					param.bounds.first().unwrap()
				{
					if let Some(path) = bound.path.segments.last() {
						if path.ident.to_string().as_str() == expected_bound {
							return Ok(TokenStream::new());
							// return Ok(param.ident.clone().into_token_stream());
						}
					}
				}
			}
		}
	}
	return Err(syn::Error::new(Span::call_site(), err));
}