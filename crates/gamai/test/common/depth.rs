use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let tree = || {
		tree! {
			<empty_node>//1
				<empty_node/>
				<empty_node>//1
					<empty_node/>//2
				</empty_node>
			</empty_node>
		}
	};

	expect(tree.graph_depth()).to_be(1)?;
	expect(tree.child_index()).to_be(0)?;
	// expect(tree.parent_depth()).to_be(0)?;
	expect(tree.graph_id()).to_be(1)?;

	let child = tree.child(1);
	// let child = child.child(0);
	expect(child.graph_depth()).to_be(2)?;
	expect(child.child_index()).to_be(1)?;
	// expect(child.parent_depth()).to_be(0)?;
	expect(child.graph_id()).to_be(1)?;

	// TODO pass this
	expect(tree.graph_depth()).to_be(1)?;
	expect(tree.child(1).graph_depth()).to_be(2)?;
	expect(tree.child(1).child(0).graph_depth()).to_be(3)?;

	// expect(true).to_be_false()?;

	Ok(())
}


#[sweet_test]
pub fn path_depth() -> Result<()> {
	const GRAPH_ID: usize = 7;
	type Root = TreePathRoot<GRAPH_ID>;
	type Child = TreePathSegment<0, Root>;
	type GrandChild = TreePathSegment<0, Child>;
	expect(Root::DEPTH).to_be(0)?;
	expect(Child::DEPTH).to_be(1)?;
	expect(GrandChild::DEPTH).to_be(2)?;
	expect(GrandChild::GRAPH_ID).to_be(GRAPH_ID)?;

	Ok(())
}
