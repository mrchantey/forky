use crate::*;
use bevy::prelude::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChildNodeState<C: Edge> {
	pub state: NodeState,
	marker: PhantomData<C>,
}

impl<C: Edge> Deref for ChildNodeState<C> {
	type Target = NodeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<C: Edge> DerefMut for ChildNodeState<C> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

impl<C: Edge> ChildNodeState<C> {
	pub fn new(state: NodeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChildEdgeState<C: Edge> {
	pub state: EdgeState,
	marker: PhantomData<C>,
}

impl<C: Edge> ChildEdgeState<C> {
	pub fn new(state: EdgeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

impl<C: Edge> Deref for ChildEdgeState<C> {
	type Target = EdgeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<C: Edge> DerefMut for ChildEdgeState<C> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct EdgePhantom<A: AiNode, const INDEX: usize>(PhantomData<A>);

impl<N: AiNode, const INDEX: usize> Edge for EdgePhantom<N, INDEX> {
	type NextNode = N;
	const INDEX: usize = INDEX;
}
