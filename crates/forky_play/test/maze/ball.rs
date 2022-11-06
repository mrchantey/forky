use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math, *};
use forky_play::maze::*;
use forky_play::OptI32X;
use forky_play::*;
use sweet::*;

sweet! {
	it "works" {
		app::init()
				.add_startup_system(utility::surrender_focus)
				.insert_resource(RapierConfiguration::with_gravity_scalar(10.))
				.add_event::<RespawnEvent>()
				.add_event::<DespawnEvent>()
				.add_system(maze_3d::respawn)
				.add_system(maze_3d::despawn)
				.add_system(ball::respawn)
				.add_system(ball::despawn_on_ball_fall)
				.add_startup_system(spawn)
				.run();
	}
}

pub fn spawn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let obj = commands
		.spawn_bundle(PbrBundle {
			transform: Transform::from_xyz(0., -0.1, 0.)
				.with_scale_xyz(10., 0.2, 10.)
				.with_rotation_x(0.1),
			mesh: meshes.add(Mesh::from(shape::Cube::default())),
			material: materials.from_white(),
			..default()
		})
		.insert(Collider::cuboid(0.5, 0.5, 0.5))
		.id();
}
