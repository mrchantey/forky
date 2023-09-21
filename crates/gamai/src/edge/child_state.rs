use crate::*;
use bevy_ecs::prelude::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

// pub type DerefEdge<'a> = Mut<'_, gamai::ChildNodeState<Child0>>;
// pub type DerefNode<'a> = Mut<'_, gamai::ChildNodeState<Child0>>;
pub type DerefEdge<'a> = &'a mut dyn std::ops::DerefMut<Target = EdgeState>;
pub type DerefNode<'a> = &'a mut dyn std::ops::DerefMut<Target = NodeState>;
// pub type OptDerefNode<'a> = Option<&'a dyn std::ops::Deref<Target = NodeState>>;

// #[derive(Clone)]
pub struct ChildState<'a> {
	pub index: usize,
	pub entity: Entity,
	// pub edge: Mut<'a, gamai::ChildNodeState<Child0>>,
	pub edge: DerefEdge<'a>,
	pub node: Option<DerefNode<'a>>,
	// pub node: Option<DerefNode<'a>>,
	pub node_state_func: fn(&mut Commands, Entity, state: NodeState),
}


impl<'a> ChildState<'a> {
	/// helper function for adding a child node state from a parent where the concrete type is not known.
	/// if the state already exists in the entity it will be updated, otherwise a command will be created.
	pub fn set_node_state(
		&mut self,
		commands: &mut Commands,
		state: NodeState,
	) {
		if let Some(val) = &mut self.node {
			***val = state;
		} else {
			(self.node_state_func)(commands, self.entity, state);
		}
	}
}


#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChildNodeState<N: AiNode> {
	pub state: NodeState,
	marker: PhantomData<N>,
}

impl<N: AiNode> Deref for ChildNodeState<N> {
	type Target = NodeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<N: AiNode> DerefMut for ChildNodeState<N> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

impl<N: AiNode> ChildNodeState<N> {
	pub fn new(state: NodeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChildEdgeState<N: AiNode> {
	pub state: EdgeState,
	marker: PhantomData<N>,
}

impl<N: AiNode> ChildEdgeState<N> {
	pub fn new(state: EdgeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

impl<N: AiNode> Deref for ChildEdgeState<N> {
	type Target = EdgeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<N: AiNode> DerefMut for ChildEdgeState<N> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}
