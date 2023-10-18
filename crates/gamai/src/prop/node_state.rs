use std::fmt::Debug;

/// Marker prop indicating a node is currently running.
/// 
/// This gets removed automatically when a [NodeState] is added.
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
