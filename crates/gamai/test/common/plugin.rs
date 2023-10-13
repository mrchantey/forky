use bevy_app::App;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn it_works() -> Result<()> {
	let my_tree = || tree! {<node_always_succeed/>};

	let mut app = App::new();

	app.add_plugins(AiPlugin::new(my_tree));

	let entity = app.world.spawn(AiBundle::new(my_tree)).id();

	expect(my_tree.node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;

	app.update();

	expect(my_tree.node_state(&app.world, entity))
		.to_be(Some(NodeState::Success))?;

	Ok(())
}
