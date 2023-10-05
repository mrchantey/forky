use super::*;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use rstml::Parser;
use rstml::ParserConfig;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::Result;

static GRAPH_ID: AtomicUsize = AtomicUsize::new(0);

pub fn parse_tree(tokens: proc_macro::TokenStream) -> Result<TokenStream> {
	let config = ParserConfig::new()
		.recover_block(true)
		.number_of_top_level_nodes(1);

	let (ident, tokens) = try_get_ident(tokens.into());


	let parser = Parser::new(config);
	let (nodes_rsx, errors) = parser.parse_recoverable(tokens).split_vec();

	if let Some(node_rsx) = nodes_rsx.first() {
		let graph_id = GRAPH_ID.fetch_add(1, Ordering::SeqCst);
		let root = NodeConfig::root(node_rsx, graph_id)?;
		let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());

		let out = if let Some(ident) = ident {
			root.to_struct(ident)
		} else {
			root.to_instance()
		};

		Ok(quote! {
			#(#errors;)*
			#out
		})
	} else {
		Err(syn::Error::new(Span::call_site(), "Expected a root node"))
	}
}

fn try_get_ident(tokens: TokenStream) -> (Option<Ident>, TokenStream) {
	let mut iter = tokens.into_iter().peekable();
	let ident = match iter.peek() {
		Some(TokenTree::Ident(ident)) => Some(ident.clone()),
		_ => None,
	};
	if ident.is_some() {
		// remove ident & next comma
		let _ = iter.next();
		let _ = iter.next();
	}
	(ident, iter.collect())
}
