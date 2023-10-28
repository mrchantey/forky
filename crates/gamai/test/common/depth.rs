use gamai::node::*;
use sweet::*;

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
