#![feature(
	let_chains,
	return_position_impl_trait_in_trait,//required for users, stablize pr open https://github.com/rust-lang/rust/pull/115822
	associated_type_defaults,
	associated_type_bounds,
	associated_const_equality,
	impl_trait_in_fn_trait_return,
	fn_traits,
	// inherent_associated_types,
	generic_const_exprs,
)]
// suppress generic_const_exprs warning
#![allow(incomplete_features)]
//allow proc macros to work internally
extern crate self as gamai;
pub use gamai_macros::*;
mod node;
pub use self::node::*;
mod builtin_nodes;
pub use self::builtin_nodes::*;
pub use anyhow::bail;
pub use anyhow::Result;
