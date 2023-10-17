use bevy_app::App;
use gamai::builtin_nodes::*;
use gamai::*;
use sweet::*;


#[sweet_test]
pub fn always_fail() -> Result<()> {
	let my_tree = || tree! {	<node_always_fail/>};

	let mut app = App::new();
	app.add_plugins(AiPlugin::new(my_tree));
	let entity = app.world.spawn(PropBundle::root(my_tree, Running)).id();

	app.update();

	expect(Prop::<NodeState, _>::get(my_tree, &app.world, entity))
		.to_be(Some(&NodeState::Failure))?;

	Ok(())
}
#[sweet_test]
pub fn always_succeed() -> Result<()> {
	let my_tree = || tree! {	<node_always_succeed/>};

	let mut app = App::new();
	app.add_plugins(AiPlugin::new(my_tree));
	let entity = app.world.spawn(PropBundle::root(my_tree, Running)).id();

	app.update();

	expect(Prop::<NodeState, _>::get(my_tree, &app.world, entity))
		.to_be(Some(&NodeState::Success))?;

	Ok(())
}
