use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let tree = tree! {
		<empty_node>
			<empty_node/>
			<empty_node>
				<empty_node/>
			</empty_node>
		</empty_node>
	};

	expect(tree.graph_depth()).to_be(1)?;
	expect(tree.child_index()).to_be(0)?;
	expect(tree.parent_depth()).to_be(0)?;
	expect(tree.graph_id()).to_be(1)?;

	let child = tree.child(1);
	// let child = child.child(0);
	expect(child.graph_depth()).to_be(2)?;
	expect(child.child_index()).to_be(1)?;
	expect(child.parent_depth()).to_be(0)?;
	expect(child.graph_id()).to_be(1)?;

	// expect(true).to_be_false()?;

	Ok(())
}
