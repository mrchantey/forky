use bevy_app::prelude::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let my_tree = || {
		tree! {
			<sequence apply_deferred>
				<node_always_succeed apply_deferred/>
				<node_always_succeed apply_deferred/>
				<node_always_succeed apply_deferred/>
				<node_always_succeed apply_deferred/>
			</sequence>
		}
	};

	let mut app = App::new();

	app.add_plugins(AiPlugin::new(my_tree));
	let entity = app.world.spawn(Prop::from_node(my_tree, Running)).id();

	app.update();

	let out = my_tree.get_recursive::<Running>(&app.world, entity);
	expect(out.value).to_be_some()?;
	let out = my_tree.get_recursive::<NodeState>(&app.world, entity);
	expect(out.children[0].value).to_be_some()?;
	expect(out.children[1].value).to_be_none()?;
	
	app.update();
	
	let out = my_tree.get_recursive::<Running>(&app.world, entity);
	expect(out.value).to_be_some()?;
	let out = my_tree.get_recursive::<NodeState>(&app.world, entity);
	expect(out.children[1].value).to_be_some()?;
	
	app.update();
	// app.update();
	// app.update();
	// app.update();
	let out = my_tree.get_recursive::<Running>(&app.world, entity);
	expect(out.value).to_be_some()?;
	Ok(())
}
