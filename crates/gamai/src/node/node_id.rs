// use super::*;
// use std::marker::PhantomData;

/// Trait used to distinguish node instances.
pub trait IntoNodeId: 'static + Send + Sync + Default {
	const GRAPH_ID: usize;
	const GRAPH_DEPTH: usize;
	const CHILD_INDEX: usize;
	const NODE_ID: usize;

	fn graph_id(&self) -> usize { Self::GRAPH_ID }
	fn graph_depth(&self) -> usize { Self::GRAPH_DEPTH }
	fn child_index(&self) -> usize { Self::CHILD_INDEX }
	fn node_id(&self) -> usize { Self::NODE_ID }
}
