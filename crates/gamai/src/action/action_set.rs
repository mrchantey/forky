use crate::node::*;
use bevy_ecs::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;


#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ActionOrder {
	/// Run before parent update.
	PreParentUpdate,
	/// Run after parent update, before update.
	PreUpdate,
	/// Default update stage for actions.
	#[default]
	Update,
	/// Run after update, before child update.
	PostUpdate,
}

#[derive(SystemSet, Clone, Eq, PartialEq, Hash)]
pub struct ActionSet {
	pub order: ActionOrder,
	pub graph_id: usize,
	pub depth: usize,
	/// internal, used to distinguish sets
	pub parent: Option<Box<ActionSet>>,
}


impl ActionSet {
	pub fn pre_parent_update<P: TreePath>() -> Self {
		Self::new::<P>(ActionOrder::PreParentUpdate)
	}
	pub fn pre_update<P: TreePath>() -> Self {
		Self::new::<P>(ActionOrder::PreUpdate)
	}
	pub fn update<P: TreePath>() -> Self { Self::new::<P>(ActionOrder::Update) }
	pub fn post_update<P: TreePath>() -> Self {
		Self::new::<P>(ActionOrder::PostUpdate)
	}

	pub fn new<P: TreePath>(order: ActionOrder) -> Self {
		let parent = if P::DEPTH == 0 {
			None
		} else {
			Some(Box::new(ActionSet::new::<P::Parent>(order)))
		};
		Self {
			parent,
			order,
			graph_id: P::GRAPH_ID,
			depth: P::DEPTH,
		}
	}
}

impl Debug for ActionSet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ActionSet")
			.field("order", &self.order)
			.field("graph_id", &self.graph_id)
			.field("depth", &self.depth)
			.finish()
	}
}
