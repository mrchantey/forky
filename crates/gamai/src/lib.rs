#![feature(
	let_chains,
	return_position_impl_trait_in_trait,//required for users, stablize pr open https://github.com/rust-lang/rust/pull/115822
	associated_type_defaults,
	associated_type_bounds,
	// inherent_associated_types,
	// generic_const_exprs,
)]
//allow proc macros to work internally
extern crate self as gamai;
pub use gamai_macros::*;
mod node;
pub use self::node::*;
mod builtin_nodes;
pub use self::builtin_nodes::*;
pub use anyhow::bail;
pub use anyhow::Result;
