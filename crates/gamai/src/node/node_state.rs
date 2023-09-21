use std::fmt::Debug;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
	#[default]
	Running,
	Success,
	Failure,
}