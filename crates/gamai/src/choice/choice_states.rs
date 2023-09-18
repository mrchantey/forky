use crate::*;
use bevy::prelude::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

// derive debug for `ChildNodeState` etc
pub trait Choice: std::fmt::Debug + 'static + Send + Sync {
	const INDEX: usize;
	type AiNode: AiNode;
}

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChildNodeState<C: Choice> {
	pub state: NodeState,
	marker: PhantomData<C>,
}

impl<C: Choice> Deref for ChildNodeState<C> {
	type Target = NodeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<C: Choice> DerefMut for ChildNodeState<C> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

impl<C: Choice> ChildNodeState<C> {
	pub fn new(state: NodeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChoiceEdgeState<C: Choice> {
	pub state: EdgeState,
	marker: PhantomData<C>,
}

impl<C: Choice> ChoiceEdgeState<C> {
	pub fn new(state: EdgeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

impl<C: Choice> Deref for ChoiceEdgeState<C> {
	type Target = EdgeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<C: Choice> DerefMut for ChoiceEdgeState<C> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct ChoicePhantom<A: AiNode, const INDEX: usize>(PhantomData<A>);

impl<A: AiNode, const INDEX: usize> Choice for ChoicePhantom<A, INDEX> {
	type AiNode = A;
	const INDEX: usize = INDEX;
}
