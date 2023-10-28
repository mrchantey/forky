use bevy_ecs::prelude::*;
use gamai::common_actions::*;
use gamai::common_selectors::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn it_works() -> Result<()> {

	let my_tree = ||tree! {
		<sequence props=(true,3usize)>
			<node_always_succeed/>
		</sequence>
	};

	let mut world = World::new();
	let entity = world.spawn(TreeBundle::new(my_tree));

	Ok(())
}
