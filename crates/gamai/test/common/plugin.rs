use bevy_app::App;
use gamai::common_actions::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn it_works() -> Result<()> {
	let my_tree = || tree! {<node_always_succeed/>};

	let mut app = App::new();

	app.add_plugins(TreePlugin::new(my_tree));

	let entity = app.world.spawn(TreeBundle::new(my_tree)).id();

	let out = PropTree::<Running>::new(my_tree, &app.world, entity);

	expect(out.value).to_be_some()?;

	app.update();

	let out = PropTree::<ActionResult>::new(my_tree, &app.world, entity);
	expect(out.value).to_be(Some(&ActionResult::Success))?;

	Ok(())
}
