use crate::*;

// derive debug for `ChildNodeState` etc
pub trait AiEdge: std::fmt::Debug + 'static + Send + Sync {
	const INDEX: usize;
	type NextNode: AiNode;
}
