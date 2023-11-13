use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// used for distinguishing system sets etc
/// so must be hash, equal, send
pub trait TreePath:
	'static + Send + Sync + Sized + Eq + Hash + Clone + Debug
{
	type Parent: TreePath;
	
	type Root: TreePath = <Self::Parent as TreePath>::Root;
	const CHILD_INDEX: usize;
	const GRAPH_ID: usize = Self::Parent::GRAPH_ID;
	const DEPTH: usize = Self::Parent::DEPTH + 1;

	fn as_string() -> String {
		format!(
			"depth: {}, child_index: {}, graph_id: {}",
			Self::DEPTH,
			Self::CHILD_INDEX,
			Self::GRAPH_ID
		)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TreePathRoot<const GRAPH_ID: usize>;
impl<const GRAPH_ID: usize> TreePath for TreePathRoot<GRAPH_ID> {
	//careful here, could cause infinite loops
	type Parent = Self;
	type Root = Self;
	const CHILD_INDEX: usize = 0;
	const GRAPH_ID: usize = GRAPH_ID;
	const DEPTH: usize = 0;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TreePathSegment<const CHILD_INDEX: usize, Parent: TreePath> {
	phantom: PhantomData<Parent>,
}

impl<const CHILD_INDEX: usize, Parent: TreePath> TreePath
	for TreePathSegment<CHILD_INDEX, Parent>
{
	type Parent = Parent;
	const CHILD_INDEX: usize = CHILD_INDEX;
}

impl<const CHILD_INDEX: usize, Parent: TreePath>
	TreePathSegment<CHILD_INDEX, Parent>
{
	pub fn new() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}