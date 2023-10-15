use bevy_app::App;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn it_works() -> Result<()> {
	let my_tree = || tree! {<node_always_succeed/>};

	let mut app = App::new();

	app.add_plugins(AiPlugin::new(my_tree));

	let entity = app
		.world
		.spawn(PropBundle::root(my_tree, NodeState::Running))
		.id();

	let out = PropTree::new(my_tree, &app.world, entity);

	expect(out.value).to_be(Some(&NodeState::Running))?;

	app.update();

	let out = PropTree::new(my_tree, &app.world, entity);
	expect(out.value).to_be(Some(&NodeState::Success))?;

	Ok(())
}
