use bevy::prelude::*;
use sweet::*;

#[derive(Debug, Component, Resource, PartialEq)]
pub struct Health(pub u32);

sweet! {
	it "works" {
		let mut app = App::new();
		let entity = app.world.spawn_empty().id();
		expect(&app).not().to_have_component::<Health>(entity)?;
		app.world.entity_mut(entity).insert(Health(7));
		expect(&app).to_have_component::<Health>(entity)?;
		expect(&app).component(entity)?.to_be(&Health(7))?;

		expect(&app).not().resource::<Health>()?;
	}
}
