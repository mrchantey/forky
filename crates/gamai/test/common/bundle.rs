use bevy_app::App;
use gamai::*;
use sweet::*;

fn my_node() -> impl AiNode {
	tree! {<empty_node/>}
}


#[sweet_test]
pub fn it_works() -> Result<()> {
	let mut app = App::new();
	let entity = app.world.spawn(AiBundle::new(my_node)).id();
	expect(my_node.node_state(&app.world, entity)).to_be_none()?;
	expect(my_node.edge_state(&app.world, entity))
		.to_be(Some(EdgeState::Fail))?;

	let entity = app.world.spawn(AiBundle::running(my_node)).id();
	expect(my_node.node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;

	Ok(())
}
