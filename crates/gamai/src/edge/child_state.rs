use crate::*;
use bevy_ecs::prelude::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Clone, PartialEq)]
pub struct ChildStates {
	pub entity: Entity,
	// pub children: Vec<(EdgeState, Option<NodeState>)>,
	pub children: Vec<EdgeState>,
}
// #[derive(Debug, Clone, PartialEq)]
// pub struct ChildState {
// 	index: usize,
// 	edge: EdgeState,
// 	node: Option<NodeState>,
// }


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
