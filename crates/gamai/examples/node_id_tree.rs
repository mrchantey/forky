#![feature(associated_const_equality)]
use gamai::*;
use std::marker::PhantomData;

struct MyTree<const CHILD_INDEX: usize, Parent: IntoNodeId = RootParent<0>> {
	// id: NodeId<CHILD_INDEX, Parent>,
	phantom: PhantomData<Parent>,
}

/// the default graph is set in the macro
impl<
		const CHILD_INDEX: usize,
		Parent: IntoNodeId<GRAPH_ID = { CHILD_INDEX }>,
	> Default for MyTree<CHILD_INDEX, Parent>
{
	fn default() -> Self {
		Self {
			// id: NodeId::default(),
			phantom: PhantomData,
		}
	}
}

fn main() {}
