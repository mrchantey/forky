use proc_macro2::Literal;
use proc_macro2::TokenTree;
use quote::quote;
use std::iter::Peekable;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;


static CNT: AtomicUsize = AtomicUsize::new(0);


#[derive(Debug, Clone)]
pub struct TestCaseFlags {
	pub id: usize,
	pub name: Literal,
	pub skip: bool,
	pub only: bool,
	pub e2e: bool,
	pub non_send: bool,
}

impl Default for TestCaseFlags {
	fn default() -> Self {
		Self {
			id: CNT.fetch_add(1, Ordering::SeqCst),
			name: Literal::string("anonymous"),
			skip: false,
			only: false,
			e2e: false,
			non_send: false,
		}
	}
}

impl TestCaseFlags {
	pub fn to_config(&self) -> proc_macro2::TokenStream {
		let skip = self.skip;
		let only = self.only;
		let context = if self.e2e {
			quote!(sweet::TestRunEnvironment::EndToEnd)
		} else {
			quote!(sweet::TestRunEnvironment::Unit)
		};

		quote! {sweet::TestCaseConfig{
				skip:#skip,
				only:#only,
				context:#context,
			}
		}
	}
}

pub fn parse_case_flags<I>(
	iter: &mut Peekable<I>,
) -> syn::parse::Result<TestCaseFlags>
where
	I: Iterator<Item = TokenTree>,
{
	let mut flags = TestCaseFlags::default();

	while let Some(t) = iter.peek() {
		match t {
			TokenTree::Literal(lit) => {
				flags.name = lit.clone();
				// let _ = iter.next().unwrap();
			}
			TokenTree::Ident(ident) => {
				let i_str = ident.to_string();
				match i_str.as_str() {
					"skip" => {
						flags.skip = true;
						// let _ = iter.next().unwrap();
					}
					"only" => {
						flags.only = true;
						// let _ = iter.next().unwrap();
					}
					"e2e" => {
						flags.e2e = true;
						// let _ = iter.next().unwrap();
					}
					"nonSend" => {
						flags.non_send = true;
					}
					_ => {
						return Err(syn::parse::Error::new(
							ident.span(),
							"Config values: skip, only, e2e, nonSend",
						));
					}
				}
			}
			_ => break,
		}
		let _ = iter.next().unwrap();
	}
	Ok(flags)
}
