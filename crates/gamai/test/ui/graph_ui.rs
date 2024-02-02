use crate::tests::action::typed_test_action_graph;
use gamai::ui::DiGraphVecNEExt;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let graph = typed_test_action_graph();
	let _roots = graph.into_field_ui_roots();

	Ok(())
}
