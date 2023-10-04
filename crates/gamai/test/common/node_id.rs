use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let root = PhantomNodeId::<0, RootParent<0>>::default();
	expect(root.graph_id()).to_be(0)?;
	expect(root.child_index()).to_be(0)?;
	expect(root.graph_depth()).to_be(1)?;
	expect(root.parent_depth()).to_be(0)?;

	let child0 = PhantomNodeId::<0, _>::new(&root);
	expect(child0.graph_id()).to_be(0)?;
	expect(child0.child_index()).to_be(0)?;
	expect(child0.graph_depth()).to_be(2)?;
	expect(child0.parent_depth()).to_be(1)?;

	let child1 = PhantomNodeId::<1, _>::new(&root);
	expect(child1.child_index()).to_be(1)?;

	Ok(())
}
