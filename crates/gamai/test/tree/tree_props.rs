use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;


#[action]
fn my_system<N: AiNode>() {}
#[tree_builder]
pub fn MyTree() -> impl TreeElement {
	tree! {
		<my_system>
			<my_system/>
		</my_system>
	}
}


#[sweet_test]
pub fn tree_props() -> Result<()> {
	let mut world = World::new();
	let _entity = world.spawn(TreeBundle::inactive(MyTree)).id();
	// expect(world.inspect_entity(entity).count()).to_be(1)?;

	Ok(())
}
