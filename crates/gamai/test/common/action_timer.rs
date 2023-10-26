use bevy_app::App;
use gamai::common_actions::*;
use gamai::common_selectors::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let my_tree = || {
		tree! {
			<sequence apply_deferred>
				<succeed_in_one_second apply_deferred/>
				<succeed_in_one_second apply_deferred/>
			</sequence>
		}
	};

	app.add_plugins(TreePlugin::new(my_tree));
	app.insert_test_timer();
	let entity = app
		.world
		.spawn((
			Prop::from_node(my_tree, Running),
			TreeBundle::recursive(my_tree, ActionTimer::default()),
		))
		.id();

	app.update_with_millis(1); //start running 1

	let out = my_tree.get_recursive::<ActionTimer>(&app.world, entity);
	expect(out.value).to_be_some()?;
	expect(out.children[0].value).to_be_some()?;
	let out = my_tree.get_recursive::<ActionResult>(&app.world, entity);
	expect(out.children[0].value).to_be_none()?;
	expect(out.children[1].value).to_be_none()?;

	app.update_with_secs(1); //end running 1

	let out = my_tree.get_recursive::<ActionResult>(&app.world, entity);
	expect(out.children[0].value).to_be_some()?;
	expect(out.children[1].value).to_be_none()?;
	let out = my_tree.get_recursive::<Running>(&app.world, entity);
	expect(out.children[0].value).to_be_none()?;
	expect(out.children[1].value).to_be_none()?;

	app.update(); //start running 2
	app.update_with_secs(1); //end running 2

	let out = my_tree.get_recursive::<ActionResult>(&app.world, entity);
	expect(out.children[0].value).to_be_none()?;
	expect(out.children[1].value).to_be_some()?;

	Ok(())
}
