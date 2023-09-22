use crate::*;
use bevy_ecs::prelude::*;
use bevy_ecs::query::WorldQuery;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum EdgeState {
	Pass,
	#[default]
	Fail,
	Weight(f32),
	// RankedWeight(u32, f32), TODO
}

impl EdgeState {
	pub fn set(&mut self, other: Self) { *self = other; }
}

impl PartialOrd for EdgeState {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let val = match (self, other) {
			(EdgeState::Fail, EdgeState::Fail) => Ordering::Equal,
			(EdgeState::Fail, _) => Ordering::Less,
			(_, EdgeState::Fail) => Ordering::Greater,
			(EdgeState::Pass, EdgeState::Pass) => Ordering::Equal,
			(EdgeState::Pass, _) => Ordering::Less,
			(_, EdgeState::Pass) => Ordering::Greater,
			(EdgeState::Weight(w1), EdgeState::Weight(w2)) => w1.total_cmp(&w2),
		};
		Some(val)
	}
}

pub type DerefEdge<'a> = &'a mut dyn std::ops::DerefMut<Target = EdgeState>;
#[derive(Debug, Default, Clone, Component, PartialEq)]
pub struct DerefEdgeState<N: AiNode> {
	pub state: EdgeState,
	pub marker: PhantomData<N>,
}

impl<N: AiNode> DerefEdgeState<N> {
	pub fn new(state: EdgeState) -> Self {
		Self {
			state,
			marker: PhantomData,
		}
	}
}

impl<N: AiNode> Deref for DerefEdgeState<N> {
	type Target = EdgeState;
	fn deref(&self) -> &Self::Target { &self.state }
}
impl<N: AiNode> DerefMut for DerefEdgeState<N> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state }
}
