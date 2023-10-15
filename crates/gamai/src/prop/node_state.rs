use crate::*;
use std::fmt::Debug;


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

pub type NodeStateProp<N> = Prop<NodeState, N>;
