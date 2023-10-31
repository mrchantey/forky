use proc_macro2::Ident;
use proc_macro2::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Error;
use syn::Expr;
use syn::Result;
use syn::Token;

const ERR: &str = "Parse Error: Expected Assignment, ie `foo = \"bar\"`";

pub fn attribute_args(
	tokens: TokenStream,
	allowed: Option<&[&str]>,
) -> Result<Vec<(Ident, Option<Expr>)>> {
	let out = Punctuated::<Expr, Token![,]>::parse_terminated
		.parse2(tokens)?
		.iter()
		.map(|item| match item {
			Expr::Assign(expr) => match expr.left.as_ref() {
				Expr::Path(path) => {
					if let Some(ident) = path.path.get_ident() {
						Ok((ident.clone(), Some(expr.right.as_ref().clone())))
					} else {
						Err(Error::new(path.span(), ERR))
					}
				}
				_ => Err(Error::new(expr.span(), ERR)),
			},
			Expr::Path(path) => {
				if let Some(ident) = path.path.get_ident() {
					Ok((ident.clone(), None))
				} else {
					Err(Error::new(path.span(), ERR))
				}
			}
			_ => Err(Error::new(item.span(), ERR)),
		})
		.collect::<Result<Vec<_>>>()?;

	if let Some(allowed) = allowed {
		for (key, _) in out.iter() {
			if false == allowed.contains(&key.to_string().as_str()) {
				let allowed =
					allowed.iter().map(|s| *s).collect::<Vec<_>>().join(", ");

				return Err(Error::new(
					key.span(),
					format!("Unknown Attribute: {key}\nExpected: {allowed}"),
				));
			}
		}
	}

	Ok(out)
}
