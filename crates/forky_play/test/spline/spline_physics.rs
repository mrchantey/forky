use bevy::prelude::*;
use forky_play::{
	spline::{CubicSpline, Spline},
	*,
};
use sweet::*;
sweet! {

	before {
		let mut app = App::new();
		app.__()
			.insert_test_timer()
			.add_plugin(spline::graph::SplineGraphPlugin)
			.add_plugin(spline::physics::SplinePhysicsPlugin)
			.__();

		let player_id = app.world.spawn((
			Transform::default(),
			spline::physics::SplinePosition::default(),
			spline::physics::SplineVelocity::default(),
			physics::AccelerationForce(Vec3::DOWN),
			Spline::Cubic(CubicSpline{
				p0: Vec3::new(0.,1.,0.),
				p1: Vec3::new(0.,0.,0.),
				p2: Vec3::new(1.,0.,0.),
				p3: Vec3::new(1.,1.,0.),
			}),
		)).id();
	}

	test "spline physics" {
		expect(app.world.get::<Transform>(player_id).unwrap().translation.y)
		.to_be(0.)?;

		app.update();
		expect(app.world.get::<Transform>(player_id).unwrap().translation.y)
		.to_be(1.)?;

		app.update_with_tick(0.1);
		expect(app.world.get::<Transform>(player_id).unwrap().translation.y)
		.to_be(0.9703)?;
	}


}
