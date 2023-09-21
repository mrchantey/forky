use std::fmt::Debug;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
	#[default]
	Running,
	Success,
	Failure,
}

impl NodeState {
	pub fn set(&mut self, other: Self) { *self = other; }
}
