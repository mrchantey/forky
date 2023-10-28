use bevy_ecs::prelude::*;
use gamai::common_actions::*;
use gamai::*;
use sweet::*;


#[sweet_test]
pub fn only_action_props() -> Result<()> {
	let my_tree = || {
		tree! {<score_always_pass/>}
	};

	let mut world = World::new();
	let entity = world.spawn(TreeBundle::inactive(my_tree)).id();

	expect(world.all_component_ids(entity).count()).to_be(1)?;

	Ok(())
}

#[sweet_test]
pub fn combined() -> Result<()> {
	let my_tree = || {
		tree! {
			<score_always_pass props=true/>
		}
	};

	let mut world = World::new();
	let entity = world.spawn(TreeBundle::inactive(my_tree)).id();

	expect(world.all_component_ids(entity).count()).to_be(2)?;

	Ok(())
}
#[sweet_test]
pub fn replace() -> Result<()> {
	let my_tree = || {
		tree! {
			<score_always_pass
				replace_props
				props=(true,3usize)
			/>
		}
	};

	let mut world = World::new();
	let entity = world.spawn(TreeBundle::inactive(my_tree)).id();

	expect(world.all_component_ids(entity).count()).to_be(2)?;

	Ok(())
}
#[sweet_test]
pub fn remove() -> Result<()> {
	let my_tree = || {
		tree! {
			<score_always_pass replace_props/>
		}
	};

	let mut world = World::new();
	let entity = world.spawn(TreeBundle::inactive(my_tree)).id();

	expect(world.all_component_ids(entity).count()).to_be(0)?;

	Ok(())
}
