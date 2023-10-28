use bevy_ecs::prelude::*;
use gamai::common_actions::*;
use gamai::*;
use sweet::*;


#[sweet_test]
pub fn prop_tree() -> Result<()> {
	let my_tree = || {
		tree! {
			<empty_node>//1
				<empty_node/>
				<empty_node>//1
					<empty_node/>//2
				</empty_node>
			</empty_node>
		}
	};

	let mut world = World::new();
	let entity = world.spawn_empty().id();
	let my_tree = PropTree::<()>::new(my_tree, &world, entity);

	expect(my_tree.depth).to_be(0)?;
	expect(my_tree.child_index).to_be(0)?;

	let child = &my_tree.children[1];
	expect(child.depth).to_be(1)?;
	expect(child.child_index).to_be(1)?;

	Ok(())
}
