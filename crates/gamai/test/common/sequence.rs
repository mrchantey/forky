use bevy_app::prelude::*;
// use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let my_tree = || {
		tree! {
			<sequence apply_deferred>
				<node_always_succeed/>
				<node_always_succeed/>
			</sequence>
		}
	};

	let mut app = App::new();

	app.add_plugins(AiPlugin::new(my_tree));
	let entity = app.world.spawn(Prop::from_node(my_tree, Running)).id();

	app.update();

	let out = my_tree.get_recursive::<Running>(&app.world, entity);
	expect(out.value).to_be_some()?;
	expect(out.children[0].value).to_be_some()?;

	app.update();
	// this looks wrong, is apply_deferred / system ordering broken?
	app.update();

	let out = my_tree.get_recursive::<Running>(&app.world, entity);
	expect(out.value).to_be_some()?;
	expect(out.children[0].value).to_be_none()?;
	// expect(out.children[0].value).to_be(Some(&NodeState::Success))?;
	expect(out.children[1].value).to_be_some()?;

	Ok(())
}
