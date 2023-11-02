use super::*;
use proc_macro2::TokenStream;
use quote::quote;
// use quote::TokenStreamExt::join;
use rstml::Parser;
use rstml::ParserConfig;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::punctuated::Punctuated;
use syn::Result;
use syn::Token;

static GRAPH_ID: AtomicUsize = AtomicUsize::new(0);

pub fn parse_tree(tokens: proc_macro::TokenStream) -> Result<TokenStream> {
	let config = ParserConfig::new().recover_block(true);
	// .number_of_top_level_nodes(1);

	let parser = Parser::new(config);
	let (nodes_rsx, errors) = parser.parse_recoverable(tokens).split_vec();
	let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());

	let out = nodes_rsx
		.into_iter()
		.map(|node_rsx| {
			let graph_id = GRAPH_ID.fetch_add(1, Ordering::SeqCst);
			let el = TreeParser::new(&node_rsx, graph_id)?.to_tokens()?;
			Ok(quote! {#el.into_root()})
		})
		.collect::<Result<Punctuated<TokenStream, Token![,]>>>()?;

	Ok(quote! {
		#(#errors;)*
		// could be we need to move into_root to call over the whole tuple
		(#out)
	})
}
