// use super::*;
// use std::marker::PhantomData;

/// Trait used to distinguish node instances.
/// The combination of `GRAPH_ID`, `CHILD_INDEX` & `GRAPH_DEPTH` is unique for each node instance.
pub trait IntoNodeId: 'static + Send + Sync {
	const GRAPH_ID: usize;
	const CHILD_INDEX: usize;
	const GRAPH_DEPTH: usize;
	const PARENT_DEPTH: usize; //required until complex expressions https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html#const-generics-with-complex-expressions

	fn graph_id(&self) -> usize { Self::GRAPH_ID }
	fn child_index(&self) -> usize { Self::CHILD_INDEX }
	fn graph_depth(&self) -> usize { Self::GRAPH_DEPTH }
	fn parent_depth(&self) -> usize { Self::PARENT_DEPTH }
}

/// used in child nodes for defining the parent to avoid cycles
#[derive(Default)]
pub struct PhantomNodeId<
	const GRAPH_ID: usize,
	const GRAPH_DEPTH: usize,
	const CHILD_INDEX: usize,
	const PARENT_DEPTH: usize,
>;

impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const PARENT_DEPTH: usize,
	> IntoNodeId
	for PhantomNodeId<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, PARENT_DEPTH>
{
	const GRAPH_ID: usize = GRAPH_ID;
	const GRAPH_DEPTH: usize = GRAPH_DEPTH;
	const CHILD_INDEX: usize = CHILD_INDEX;
	const PARENT_DEPTH: usize = PARENT_DEPTH;
}
/// The default parent for tree builders.
#[derive(Default)]
pub struct RootParent<const GRAPH_ID: usize>;
impl<const GRAPH_ID: usize> IntoNodeId for RootParent<GRAPH_ID> {
	const GRAPH_ID: usize = GRAPH_ID;
	const CHILD_INDEX: usize = 0;
	const GRAPH_DEPTH: usize = 0;
	const PARENT_DEPTH: usize = 0;
}
