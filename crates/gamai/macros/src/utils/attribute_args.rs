use proc_macro2::Ident;
use proc_macro2::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Expr;

const ERR: &str = "Parse Error: Expected Assignment, ie `foo = \"bar\"`";

pub fn attribute_args(
	tokens: TokenStream,
	allowed: Option<&[&str]>,
) -> Result<Vec<(Ident, Expr)>, syn::Error> {
	let out = Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
		.parse2(tokens)?
		.iter()
		.map(|item| match item {
			syn::Expr::Assign(expr) => match expr.left.as_ref() {
				syn::Expr::Path(path) => {
					if let Some(ident) = path.path.get_ident() {
						Ok((ident.clone(), expr.right.as_ref().clone()))
					} else {
						Err(syn::Error::new(path.span(), ERR))
					}
				}
				_ => Err(syn::Error::new(expr.span(), ERR)),
			},
			_ => Err(syn::Error::new(item.span(), ERR)),
		})
		.collect::<Result<Vec<_>, _>>()?;

	if let Some(allowed) = allowed {
		for (key, _) in out.iter() {
			if false == allowed.contains(&key.to_string().as_str()) {
				let allowed =
					allowed.iter().map(|s| *s).collect::<Vec<_>>().join(", ");

				return Err(syn::Error::new(
					key.span(),
					format!("Unknown Attribute: {key}\nExpected: {allowed}"),
				));
			}
		}
	}

	Ok(out)
}
