// use gamai::*;
// use std::marker::PhantomData;
// use sweet::*;


// /// example `#[tree]` expansion
// /// here the default graph id is set in the macro
// /// it can be overridden by using the from_parent method
// struct MyTree<const CHILD_INDEX: usize, Parent: IntoNodeId = RootParent<0>> {
// 	// id: NodeId<CHILD_INDEX, Parent>,
// 	phantom: PhantomData<Parent>,
// }

// /// the default graph is set in the macro
// impl<const CHILD_INDEX: usize, Parent: IntoNodeId> Default
// 	for MyTree<CHILD_INDEX, Parent>
// {
// 	fn default() -> Self {
// 		Self {
// 			// id: NodeId::default(),
// 			phantom: PhantomData,
// 		}
// 	}
// }

// impl<const CHILD_INDEX: usize, Parent: IntoNodeId> IntoNodeId
// 	for MyTree<CHILD_INDEX, Parent>
// {
// 	const GRAPH_ID: usize = Parent::GRAPH_ID;
// 	const CHILD_INDEX: usize = CHILD_INDEX;
// 	const GRAPH_DEPTH: usize = Parent::GRAPH_DEPTH + 1;
// 	const PARENT_DEPTH: usize = Parent::GRAPH_DEPTH;
// }

// impl<const CHILD_INDEX: usize, Parent: IntoNodeId> MyTree<CHILD_INDEX, Parent> {
// 	fn from_parent(_parent: &Parent) -> Self { Self::default() }

// 	// fn foobar() {
// 	// 	let a = NodeSet::<{ <Self as IntoNodeId>::GRAPH_ID }, { <Self as IntoNodeId>::GRAPH_DEPTH }>;
// 	// }
// }

// #[sweet_test]
// pub fn works() -> Result<()> {
// 	let root = <MyTree<0>>::default();
// 	expect(root.graph_id()).to_be(0)?;
// 	expect(root.child_index()).to_be(0)?;
// 	expect(root.graph_depth()).to_be(1)?;
// 	expect(root.parent_depth()).to_be(0)?;

// 	let child0 = <MyTree<0, _>>::from_parent(&root);
// 	expect(child0.graph_id()).to_be(0)?;
// 	expect(child0.child_index()).to_be(0)?;
// 	expect(child0.graph_depth()).to_be(2)?;
// 	expect(child0.parent_depth()).to_be(1)?;

// 	let child1 = <MyTree<1, _>>::from_parent(&root);
// 	expect(child1.child_index()).to_be(1)?;

// 	Ok(())
// }
