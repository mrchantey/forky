use bevy_app::prelude::*;
// use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let my_tree = tree! {
		<sequence>
			<node_always_succeed/>
			<node_always_succeed/>
		</sequence>
	};

	let mut app = App::new();

	app.add_plugins(my_tree.plugin());
	let entity = app.world.spawn(my_tree.bundle()).id();

	app.update();

	expect(my_tree.child(0).node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;
	expect(my_tree.child(1).node_state(&app.world, entity)).to_be_none()?;

	app.update();

	expect(my_tree.child(0).node_state(&app.world, entity)).to_be_none()?;
	expect(my_tree.child(1).node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;

	Ok(())
}
