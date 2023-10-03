use crate::*;
use bevy_ecs::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
	#[default]
	/// Set by a parent node to indicate running
	Running,
	/// Set by a child node to indicate success
	Success,
	/// Set by a child node to indicate failure
	Failure,
	// Interrupt,
}

impl NodeState {
	pub fn set(&mut self, other: Self) { *self = other; }
}

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct DerefNodeState<N: AiNode> {
	pub state: NodeState,
	pub marker: PhantomData<N>,
}

// impl<N: AiNode> DerefNodeState<N> {

// }

pub type DerefNode<'a> = &'a mut dyn std::ops::DerefMut<Target = NodeState>;

impl<N: AiNode> Deref for DerefNodeState<N> {
	type Target = NodeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<N: AiNode> DerefMut for DerefNodeState<N> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

impl<N: AiNode> DerefNodeState<N> {
	pub fn new(state: NodeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}
