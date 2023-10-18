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
//! 	app.add_plugins(TreePlugin::new(MyTree));
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
	// generic_const_exprs,
)]
// suppress generic_const_exprs warning
#![allow(incomplete_features)]
//allow proc macros to work internally
extern crate self as gamai;
pub use gamai_macros::*;

/// A node is a container for actions in a tree.
pub mod node;
#[doc(inline)]
pub use node::AiNode;
#[doc(inline)]
pub use node::IntoNode;
#[doc(inline)]
pub use node::TreePlugin;

/// A prop is a per-node `Bevy::Component`.
pub mod prop;
#[doc(inline)]
pub use prop::NodeState;
#[doc(inline)]
pub use prop::Prop;
#[doc(inline)]
pub use prop::PropBundle;
#[doc(inline)]
pub use prop::PropTree;
#[doc(inline)]
pub use prop::Running;
#[doc(inline)]
pub use prop::Score;

/// Collection of commonly used actions
pub mod common_actions;
/// Collection of commonly used selectors
pub mod common_selectors;

/// Re-exports for macros
pub mod exports {
	pub use anyhow::bail;
	pub use anyhow::Result;
	pub use bevy_ecs::prelude::*;
	pub use bevy_ecs::query::WorldQuery;
	pub use bevy_ecs::schedule::SystemConfigs;
	// currently no macros depend on bevy_app
	// pub use bevy_app::prelude::*;
}
