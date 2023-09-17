#![feature(
	let_chains,
	inherent_associated_types,
	return_position_impl_trait_in_trait,
	associated_type_defaults,
	associated_type_bounds,
	generic_const_exprs
)]
//allow proc macros to work internally
extern crate self as gamai;
pub use gamai_macros::*;
mod choice;
pub use self::choice::*;
mod agent;
pub use self::agent::*;
