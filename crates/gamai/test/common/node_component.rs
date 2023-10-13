use bevy_app::prelude::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let tree = || {
		tree! {
			<sequence>
			<sequence/>
			</sequence>
		}
	};

	let mut app = App::new();
	let entity = app.world.spawn(AiBundle::new(tree)).id();
	let value = Lifecycle::get(&tree.into_node(), &app.world, entity);
	println!("the value is: {:?}", value);
	Ok(())
}
