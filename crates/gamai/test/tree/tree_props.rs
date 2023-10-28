use bevy_ecs::prelude::*;
use gamai::common_actions::*;
use gamai::*;
use sweet::*;


#[sweet_test]
pub fn tree_props() -> Result<()> {
	let _my_tree = |val: u32| {
		tree! {<score_always_pass props=val/>}
	};

	let mut _world = World::new();
	// let _entity = world.spawn(TreeBundle::inactive(my_tree)).id();
	// expect(world.all_component_ids(entity).count()).to_be(1)?;

	Ok(())
}
