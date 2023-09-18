use crate::*;
use bevy::prelude::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChildNodeState<C: AiEdge> {
	pub state: NodeState,
	marker: PhantomData<C>,
}

impl<C: AiEdge> Deref for ChildNodeState<C> {
	type Target = NodeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<C: AiEdge> DerefMut for ChildNodeState<C> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

impl<C: AiEdge> ChildNodeState<C> {
	pub fn new(state: NodeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct ChildEdgeState<C: AiEdge> {
	pub state: EdgeState,
	marker: PhantomData<C>,
}

impl<C: AiEdge> ChildEdgeState<C> {
	pub fn new(state: EdgeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

impl<C: AiEdge> Deref for ChildEdgeState<C> {
	type Target = EdgeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<C: AiEdge> DerefMut for ChildEdgeState<C> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct EdgePhantom<A: AiNode, const INDEX: usize>(PhantomData<A>);

impl<N: AiNode, const INDEX: usize> AiEdge for EdgePhantom<N, INDEX> {
	type NextNode = N;
	const INDEX: usize = INDEX;
}
