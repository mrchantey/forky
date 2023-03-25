use bevy::prelude::*;
// use bevy::utils::Duration;
use forky_play::{physics::*, *};
use std::time::{Duration, Instant};
use sweet::*;


sweet! {

	before {
		let mut app = App::new();
		app.__()
			.insert_test_timer()
			.add_plugin(physics::EulerPhysicsPlugin)
			.__();
	}

	test "velocity" {
		let player_id = app.world.spawn((
			Transform::default(),
			Velocity(Vec3::RIGHT),
		)).id();

		expect(app.world.get::<Transform>(player_id).unwrap().translation.x)
			.to_be(0.)?;

		app.update_with_tick(1.);
		expect(app.world.get::<Transform>(player_id).unwrap().translation.x)
			.to_be(1.)?;
	}
	test "velocity" {
		let player_id = app.world.spawn((
			Transform::default(),
			Velocity(Vec3::RIGHT),
			Friction(0.5),
		)).id();

		expect(app.world.get::<Transform>(player_id).unwrap().translation.x)
			.to_be(0.)?;

		app.update_with_tick(1.);
		expect(app.world.get::<Transform>(player_id).unwrap().translation.x)
			.to_be(0.5)?;
	}

	test "acceleration - impulse"{
		let player_id = app.world.spawn((
			Transform::default(),
			AccelerationImpulse(Vec3::RIGHT),
			Velocity::default(),
		)).id();

		app.update_with_tick(2.);
		expect(app.world.get::<Velocity>(player_id).unwrap().x)
			.to_be(1.)?;
		expect(app.world.get::<Transform>(player_id).unwrap().translation.x)
			.to_be(2.)?;
	}
	test "acceleration - force"{
		let player_id = app.world.spawn((
			Transform::default(),
			AccelerationForce(Vec3::RIGHT),
			Velocity::default(),
		)).id();

		// for i in 0..20{
		// 	app.update_with_tick(0.1);
		// }
		app.update_with_tick(2.);
		expect(app.world.get::<Velocity>(player_id).unwrap().x)
			.to_be(2.)?;
		expect(app.world.get::<Transform>(player_id).unwrap().translation.x)
			.to_be(4.)?;
	}
	test skip "acceleration - force - visualize"{
		App::new()
		.forky_exit_after(4.)
		.add_plugin(plugins::CustomDefaultPlugin)
		.add_plugin(physics::EulerPhysicsPlugin)
		.add_startup_system(plugins::spawn_default_camera)
		.add_startup_system(spawn_falling_cube)
		.run();

	}
}

fn spawn_falling_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			..default()
		},
		AccelerationForce(Vec3::new(0., -9.81 * 0.1, 0.)),
		Velocity::default(),
	));
}
