use std::fmt::Debug;

/// Marker prop indicating a node is currently running.
///
/// This gets removed automatically when a [ActionResult] is added.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Running;

/// Prop added to actions to indicate their result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionResult {
	/// Set by an action to indicate success
	Success,
	/// Set by an action to indicate failure
	Failure,
	// Interrupt,
}
