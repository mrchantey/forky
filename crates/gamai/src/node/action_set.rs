use super::*;
use bevy_ecs::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ActionStage {
	/// Run before parent update.
	PreParentUpdate,
	/// Run after parent update, before update.
	PreUpdate,
	/// Default update stage for actions.
	Update,
	/// Run after update, before child update.
	PostUpdate,
}

#[derive(SystemSet, Clone, Eq, PartialEq, Hash)]
pub struct ActionSet {
	pub stage: ActionStage,
	pub graph_id: usize,
	pub depth: usize,
	/// internal, used to distinguish sets
	pub parent: Option<Box<ActionSet>>,
}


impl ActionSet {
	pub fn pre_parent_update<P: TreePath>() -> Self {
		Self::new::<P>(ActionStage::PreParentUpdate)
	}
	pub fn pre_update<P: TreePath>() -> Self {
		Self::new::<P>(ActionStage::PreUpdate)
	}
	pub fn update<P: TreePath>() -> Self { Self::new::<P>(ActionStage::Update) }
	pub fn post_update<P: TreePath>() -> Self {
		Self::new::<P>(ActionStage::PostUpdate)
	}

	pub fn new<P: TreePath>(stage: ActionStage) -> Self {
		let parent = if P::DEPTH == 0 {
			None
		} else {
			Some(Box::new(ActionSet::new::<P::Parent>(stage)))
		};
		Self {
			parent,
			stage,
			graph_id: P::GRAPH_ID,
			depth: P::DEPTH,
		}
	}
}

impl Debug for ActionSet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ActionSet")
			.field("stage", &self.stage)
			.field("graph_id", &self.graph_id)
			.field("depth", &self.depth)
			.finish()
	}
}
