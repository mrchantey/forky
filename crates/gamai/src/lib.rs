#![feature(
	let_chains,
	return_position_impl_trait_in_trait,//required for users, stablize pr open https://github.com/rust-lang/rust/pull/115822
	associated_type_defaults,
	associated_type_bounds,
	// inherent_associated_types,
	// generic_const_exprs
)]
//allow proc macros to work internally
extern crate self as gamai;
pub use gamai_macros::*;
mod edge;
pub use self::edge::*;
mod node;
pub use self::node::*;
// use anyhow;
pub use anyhow::Result;
pub use anyhow::bail;
