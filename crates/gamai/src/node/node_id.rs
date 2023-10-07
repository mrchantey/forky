// use super::*;
// use std::marker::PhantomData;

/// Trait used to distinguish node instances.
pub trait IntoNodeId: 'static + Send + Sync {
	const GRAPH_ID: usize;
	const GRAPH_DEPTH: usize;
	const CHILD_INDEX: usize;
	const NODE_ID:usize;
	const PARENT_DEPTH: usize; //required until complex expressions https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html#const-generics-with-complex-expressions

	fn graph_id(&self) -> usize { Self::GRAPH_ID }
	fn graph_depth(&self) -> usize { Self::GRAPH_DEPTH }
	fn child_index(&self) -> usize { Self::CHILD_INDEX }
	fn node_id(&self) -> usize { Self::NODE_ID }
	fn parent_depth(&self) -> usize { Self::PARENT_DEPTH }
}