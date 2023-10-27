use bevy_app::prelude::*;
use gamai::common_actions::*;
use gamai::common_selectors::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn delayed_commands() -> Result<()> {
	let my_tree = || {
		tree! {
			<sequence>
				<sequence>
					<sequence>
						<node_always_succeed/>
						<node_always_succeed/>
					</sequence>
				</sequence>
			</sequence>
		}
	};

	let mut app = App::new();

	app.add_plugins(TreePlugin::new(my_tree));
	let entity = app.world.spawn(TreeBundle::new(my_tree)).id();

	app.update();

	let out = my_tree.get_recursive::<Running>(&app.world, entity);
	expect(out.value).to_be_some()?;
	// the first child got set at the end of Update
	expect(out.children[0].value).to_be_some()?;
	expect(out.children[0].children[0].value).to_be_none()?;


	Ok(())
}

#[sweet_test]
pub fn apply_deferred() -> Result<()> {
	let my_tree = || {
		tree! {
			<sequence apply_deferred>
				<sequence apply_deferred>
					<sequence apply_deferred>
						<node_always_succeed/>
						<node_always_succeed/>
					</sequence>
				</sequence>
			</sequence>
		}
	};

	let mut app = App::new();

	app.add_plugins(TreePlugin::new(my_tree));
	let entity = app.world.spawn(TreeBundle::new(my_tree)).id();

	app.update();

	let out = my_tree.get_recursive::<Running>(&app.world, entity);
	// println!("{out}");
	expect(out.value).to_be_some()?;
	expect(out.children[0].value).to_be_some()?;
	expect(out.children[0].children[0].value).to_be_some()?;


	Ok(())
}
