use std::fmt::Debug;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Running;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
	#[default]
	/// Set by a child node to indicate success
	Success,
	/// Set by a child node to indicate failure
	Failure,
	// Interrupt,
}
