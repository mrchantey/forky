use bevy_app::App;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn it_works() -> Result<()> {
	let my_node = tree! {<empty_node/>};
	let mut app = App::new();
	// let node = my_node.node().node_state(world, entity);
	let entity = app.world.spawn(my_node.bundle()).id();
	expect(my_node.node().node_state(&app.world, entity)).to_be_none()?;
	expect(my_node.node().edge_state(&app.world, entity))
		.to_be(Some(EdgeState::Fail))?;

	let entity = app.world.spawn(my_node.bundle_running()).id();
	expect(my_node.node().node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;

	Ok(())
}
