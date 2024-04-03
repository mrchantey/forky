use bevy::prelude::*;
use sweet::*;

#[derive(Debug, PartialEq, Component, Resource)]
pub struct Health(pub u32);

sweet! {

	test "world"{
		let mut world = World::new();
		expect(&world).not().to_contain_resource::<Health>()?;
		world.insert_resource(Health(5));
		expect(&world).to_contain_resource::<Health>()?;
	}

	test "app" {
		let mut app = App::new();
		let entity = app.world_mut().spawn_empty().id();

		expect(&app).not().to_have_component::<Health>(entity)?;
		app.world_mut().entity_mut(entity).insert(Health(7));
		expect(&app).to_have_component::<Health>(entity)?;
		expect(&app).component(entity)?.to_be(&Health(7))?;

		expect(&app).not().to_contain_resource::<Health>()?;
		app.world_mut().insert_resource(Health(5));
		expect(&app).to_contain_resource::<Health>()?;
		expect(&app).resource()?.to_be(&Health(5))?;

		expect(&app).not().to_contain_non_send_resource::<Health>()?;
		app.world_mut().insert_non_send_resource(Health(5));
		expect(&app).to_contain_non_send_resource::<Health>()?;
		expect(&app).non_send_resource()?.to_be(&Health(5))?;
	}
}
