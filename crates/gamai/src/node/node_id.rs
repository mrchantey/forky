// use super::*;
use std::marker::PhantomData;

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

// TODO i think only useful for tests
pub struct NodeId<const CHILD_INDEX: usize, Parent: IntoNodeId> {
	phantom: PhantomData<Parent>,
}

impl<const CHILD_INDEX: usize, Parent: IntoNodeId> NodeId<CHILD_INDEX, Parent> {
	pub const CHILD_INDEX: usize = CHILD_INDEX;
	pub const GRAPH_ID: usize = Parent::GRAPH_ID;
	pub const GRAPH_DEPTH: usize = Parent::GRAPH_DEPTH + 1;
	pub const PARENT_DEPTH: usize = Parent::GRAPH_DEPTH;
	pub fn new(_parent: &Parent) -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}

impl<const CHILD_INDEX: usize, Parent: IntoNodeId> IntoNodeId
	for NodeId<CHILD_INDEX, Parent>
{
	const GRAPH_ID: usize = Self::GRAPH_ID;
	const CHILD_INDEX: usize = Self::CHILD_INDEX;
	const GRAPH_DEPTH: usize = Self::GRAPH_DEPTH;
	const PARENT_DEPTH: usize = Self::PARENT_DEPTH;
}

impl<const CHILD_INDEX: usize, Parent: IntoNodeId> Default
	for NodeId<CHILD_INDEX, Parent>
{
	fn default() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
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
