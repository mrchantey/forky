use proc_macro2::Ident;
use proc_macro2::TokenStream;
// use syn::*;
use quote::quote;
use sweet::*;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Expr;


#[sweet_test]
pub fn works() -> Result<()> {
	let assignments = as_assignments(quote! {foo=1,bar=(1,2,3)})?;
	expect(assignments[0].0.to_string().as_str()).to_be("foo")?;
	expect(assignments[1].0.to_string().as_str()).to_be("bar")?;

	Ok(())
}

fn as_assignments(tokens: TokenStream) -> Result<Vec<(Ident, Expr)>> {
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
	Ok(out)
}

const ERR: &str = "Expected Assignment Expression, ie `foo=(1,2,3)`";
