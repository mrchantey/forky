use crate::tests::action::*;
use gamai::ui::DiGraphVecNEExt;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let graph = test_action_graph_typed();
	let _roots = graph.into_field_ui_roots();

	Ok(())
}
