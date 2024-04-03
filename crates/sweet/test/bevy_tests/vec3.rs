use bevy::prelude::*;
use sweet::*;


#[derive(Debug, Clone, Copy, PartialEq, Deref, Component)]
struct Foo(pub Vec3);

sweet! {
	test "vec3" {
		expect(Vec3::ZERO).to_be_close_to(Vec3::ZERO)?;
		expect(Vec3::ZERO).not().to_be_close_to(Vec3::ONE)?;
		expect(&Foo(Vec3::ZERO)).to_be_close_to(Vec3::ZERO)?;
		expect(&Foo(Vec3::ZERO)).not().to_be_close_to(Vec3::new(0.2,0.2,0.2))?;
	}

	test "vec3 with app"{
		let mut app = App::new();

		let entity = app.world_mut().spawn(Foo(Vec3::ZERO)).id();
		expect(&app).component::<Foo>(entity)?
			.to_be_close_to(Vec3::ZERO)?;
	}
}
