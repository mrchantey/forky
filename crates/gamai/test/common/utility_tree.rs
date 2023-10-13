use bevy_app::App;
use gamai::*;
use sweet::*;


#[tree_builder]
pub fn MyTree() -> impl AiNode {
	tree! {
		<first_valid_edge>
			<empty_node edge=edge_always_fail/>
			<empty_node edge=edge_always_pass/>
		</first_valid_edge>
	}
}

#[sweet_test]
pub fn it_works() -> Result<()> {
	let mut app = App::new();

	app.add_plugins(AiPlugin::new(MyTree));

	let entity = app.world.spawn(AiBundle::new(MyTree)).id();
	app.update();

	expect(MyTree.child(0).node_state(&app.world, entity)).to_be_none()?;
	expect(MyTree.child(1).node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;

	Ok(())
}