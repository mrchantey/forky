use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rstml::node::KeyedAttribute;
use rstml::node::NodeElement;
use syn::spanned::Spanned;
use syn::Expr;
use syn::Result;


pub struct AttributeParser {
	pub props: Option<TokenStream>,
	pub replace_props: bool,
	pub pre_parent_update: TokenStream,
	pub pre_update: TokenStream,
	pub update_apply_deferred: bool,
	pub update: TokenStream,
	pub post_update: TokenStream,
}
impl AttributeParser {
	pub fn from_attributes(node: &NodeElement) -> Result<Self> {
		let mut attributes = Self::default();
		attributes.update = node.name().to_token_stream();


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
							attributes.update_apply_deferred = true
						}
						"before_parent" => {
							attributes.pre_parent_update = attr
								.value()
								.map(|a| a.to_token_stream())
								.ok_or_else(|| has_no_value(attr))?;
						}
						"before" => {
							attributes.pre_update = attr
								.value()
								.map(|a| a.to_token_stream())
								.ok_or_else(|| has_no_value(attr))?;
						}
						"after" => {
							attributes.post_update = attr
								.value()
								.map(|a| a.to_token_stream())
								.ok_or_else(|| has_no_value(attr))?;
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
						_ => {
							return Err(syn::Error::new(
								attr.key.span(),
								format!(
									"attribute '{}' not supported\nSupported attributes are: [before, before_parent, after, props, override_props]",
									attr.key
								),
							));
						}
					}
				}
			}
		}


		Ok(attributes)
	}

	fn parse_props(expr: &Expr) -> TokenStream {
		match expr {
			Expr::Tuple(tup) => {
				let intos = tup
					.elems
					.iter()
					.map(|e| {
						quote! {Prop::<_,Node>::new(#e),}
					})
					.collect::<TokenStream>();
				quote! {
					{struct MyStruct;
						impl gamai::prop::IntoPropBundle for MyStruct {
							fn into_bundle<Node: gamai::AiNode>(self) -> impl Bundle {
								(#intos)
							}
						}
						MyStruct
					}
				}
			}
			_ => syn::Error::new(
				expr.span(),
				"Expected Tuple, ie (Prop1::default(),Prop2::default())",
			)
			.to_compile_error(),
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
				let self_attr = self.to_action();
				// 2xtuples implement IntoPropBundle
				quote! {
					(#self_attr,#props)
				}
			}
			(false, None) => self.to_action(),
		}
	}

	pub fn to_action(&self) -> TokenStream {
		let Self {
			update,
			pre_update,
			update_apply_deferred,
			pre_parent_update,
			post_update,
			..
		} = self;
		quote! {
		gamai::node::Attributes::new(
			#pre_parent_update,
			#pre_update,
			#update,
			#update_apply_deferred,
			#post_update)
		}
	}
}
impl Default for AttributeParser {
	fn default() -> Self {
		Self {
			props: None,
			replace_props: false,
			pre_parent_update: quote!(gamai::common_actions::empty_node),
			pre_update: quote!(gamai::common_actions::empty_node),
			update_apply_deferred: false,
			update: quote!(gamai::common_actions::empty_node),
			post_update: quote!(gamai::common_actions::empty_node),
		}
	}
}
