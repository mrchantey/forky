#![feature(associated_type_bounds)]
use proc_macro::TokenStream;
mod tree;
use tree::*;
mod node_def;
use node_def::*;
mod action;
use action::*;


/// Used to define number of children allowed per node, defaults to `16`.
///
/// Use this macro to extend the number of allowed children,
/// note that they must be in scope when defining nodes that have over 16 children.
///
/// # Example
///
/// ```rust
/// gamai::define_node!(17);
/// gamai::define_node!(18);
/// gamai::define_node!(19);
/// gamai::define_node!(20);
///
#[proc_macro]
pub fn define_node(attr: TokenStream) -> TokenStream { parse_node(attr) }

/// An action is a `bevy::system` that accepts a generic `AiNode` type parameter.
/// 
/// # Example
/// ```rust
/// #[action]
/// pub fn score_always_pass<N: AiNode>(mut query: Query<&mut Prop<Score, N>>) {
/// 	for mut score in query.iter_mut() {
/// 		**score = Score::Pass;
/// 	}
/// }
/// ```
#[proc_macro_attribute]
pub fn action(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_action(attr, item)
}


/// Macro used for defining trees in RSX format.
/// 
/// # Example
/// 
/// ```rust
/// let my_tree = || tree!{
/// 	<sequence>
/// 		<empty_node>
/// 	</sequence>
/// };
/// 
/// ```
#[proc_macro]
pub fn tree(item: TokenStream) -> TokenStream {
	parse_tree(item)
	.unwrap_or_else(syn::Error::into_compile_error)
	.into()
}

/// A tree is a function that returns `impl IntoNode`
#[proc_macro_attribute]
pub fn tree_builder(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_tree_builder(attr, item)
}

#[doc(hidden)]
#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream { html_inner(tokens, false) }
