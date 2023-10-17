//! A flexible task switching library suitable for game AI & robotics.
//!
//! # Usage
//! ```rust
//! use gamai::*;
//!
//! #[action]
//! fn say_hello<N: AiNode>(query: Query<Entity, With<Prop<Running,N>>){
//! 	for _entity in query.iter(){
//! 		println!("this action is running!");
//! 	}
//! }
//!
//! #[tree_builder]
//! fn MyTree() -> impl AiNode {
//! 	tree! {
//! 		<sequence>
//! 			<say_hello/>
//! 			<say_hello/>
//! 		</sequence>
//! 	}
//! }
//!
//! fn main(){
//! 	let mut app = App::new();
//! 	app.add_plugins(AiPlugin::new(MyTree));
//! 	app.world.spawn(PropBundle::root(MyTree, Running));
//!
//! 	app.update(); // runs first child
//! 	app.update(); // runs second child
//! }
//!
//! ```
//!
//!
#![feature(
	let_chains,
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
mod prop;
pub use self::prop::*;
mod builtin_nodes;
pub use self::builtin_nodes::*;
mod selectors;
pub use self::selectors::*;
pub use anyhow::bail;
pub use anyhow::Result;
pub mod exports {
	pub use bevy_ecs::prelude::*;
	pub use bevy_ecs::query::WorldQuery;
	pub use bevy_ecs::schedule::SystemConfigs;
	// currently no macros depend on bevy_app
	// pub use bevy_app::prelude::*;
}
