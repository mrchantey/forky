use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rstml::node::KeyedAttribute;
use rstml::node::NodeElement;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::Expr;
use syn::Result;

pub struct AttributeParser<'a> {
	pub props: Option<TokenStream>,
	pub replace_props: bool,
	pub actions: TokenStream,
	pub apply_deferred: Option<TokenStream>,
	pub other_props: Vec<&'a KeyedAttribute>,
}

impl<'a> Default for AttributeParser<'a> {
	fn default() -> Self {
		Self {
			props: None,
			actions: TokenStream::new(),
			other_props: Vec::new(),
			replace_props: false,
			apply_deferred: None,
		}
	}
}

impl<'a> AttributeParser<'a> {
	pub fn from_node(node: &'a NodeElement) -> Result<Self> {
		let mut attributes = Self::default();

		let is_group = match node.name().to_string().as_str() {
			"group" => true,
			_ => false,
		};

		attributes.actions = if is_group {
			quote! {()}
		} else {
			node.name().to_token_stream()
		};

		let has_no_value = |attr: &KeyedAttribute| {
			syn::Error::new(attr.key.span(), "this attribute must have a value")
		};
		for attribute in node.attributes() {
			match attribute {
				XmlNodeAttribute::Block(block) => {
					return Err(syn::Error::new(
						block.span(),
						format!("block attributes not currently supported"),
					));
				}
				XmlNodeAttribute::Attribute(attr) => {
					match attr.key.to_string().as_str() {
						"apply_deferred" => {
							if let Some(value) = attr.value() {
								attributes.apply_deferred =
									Some(value.to_token_stream());
							} else {
								attributes.apply_deferred = Some(quote!(true));
							}
						}
						"before_parent" => {
							panic!("deprecated");
						}
						"before" => {
							panic!("deprecated");
						}
						"after" => {
							panic!("deprecated");
						}
						"props" => {
							attributes.props = Some(Self::parse_props(
								attr.value()
									.ok_or_else(|| has_no_value(attr))?,
							));
						}
						"replace_props" => {
							attributes.replace_props = true;
						}
						"actions" => {
							let map_elems =
								|elems: &Punctuated<Expr, Comma>| {
									let elems = elems
										.iter()
										.map(|e| {
											quote! {#e.into_action_config(),}
										})
										.collect::<TokenStream>();
									quote!((#elems))
								};

							attributes.actions = if let Some(expr) =
								attr.value()
							{
								match expr {
									Expr::Array(arr) => map_elems(&arr.elems),
									Expr::Tuple(tup) => map_elems(&tup.elems),
									other => {
										let action = other.to_token_stream();
										quote! {#action.into_action_config()}
									}
								}
							} else {
								quote! {()}
							}
						}
						_ => {
							attributes.other_props.push(attr);
							// return Err(syn::Error::new(
							// 	attr.key.span(),
							// 	format!(
							// 		"attribute '{}' not supported\nSupported attributes are: [before, before_parent, after, props, replace_props]",
							// 		attr.key
							// 	),
							// ));
						}
					}
				}
			}
		}


		Ok(attributes)
	}

	fn parse_props(expr: &Expr) -> TokenStream {
		let parse_arr = |elems: &Punctuated<Expr, Comma>| -> TokenStream {
			let num_props = elems.len();
			if num_props >= 16 {
				return syn::Error::new(
					expr.span(),
					"Too many props, max is 15",
				)
				.to_compile_error();
			}
			let intos = elems
				.iter()
				.map(|e| {
					quote! {#e,}
				})
				.collect::<TokenStream>();

			quote! {
				gamai::prop::RawProps((#intos))
			}
		};

		match expr {
			Expr::Tuple(tup) => {
				parse_arr(&tup.elems)
			}
			Expr::Array(arr)=>{
				parse_arr(&arr.elems)
			}
			val=> {
				quote!{gamai::prop::RawProps((#val,))}
			}
			// _ => syn::Error::new(
			// 	expr.span(),
			// 	"Expected Tuple, ie (Prop1::default(),Prop2::default())",
			// )
			// .to_compile_error(),
		}
	}

	pub fn to_prop_bundle(&self) -> TokenStream {
		match (self.replace_props, &self.props) {
			(true, Some(props)) => {
				quote! {
					#props
				}
			}
			(true, None) => {
				quote! {
					()
				}
			}

			(false, Some(props)) => {
				let actions = &self.actions;
				quote! {
					(#actions,#props)
				}
			}
			(false, None) => self.actions.clone(),
		}
	}
}
