use bevy::prelude::*;
use forky_core::{math::*, *};
use forky_play::*;
use sweet::*;

sweet! {
	it "works" {
		app::init()
			// .add_plugin(maze::MazePlugin)
			.add_startup_system(spawn_wall)
			.add_plugin(input::DebugCameraPlugin)
			// .forky_exit_after(2)
			.run();
	}
}


fn spawn_wall_horizontal(width: f32) -> Vec<Transform> {
	vec![Transform::from_scale(Vec3::new(width, 0., 0.)).with_rotation(Quat::)]
}
// fn spawn_wall_horizontal(width: f32) -> Vec<Transform> {
// 	vec![Transform::from_scale(Vec3::new(width, 0., 0.))]
// }


pub fn spawn_wall(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let parent = commands
		.spawn_bundle(SpatialBundle {
			transform: Transform::from_xyz(0., 0., 0.).with_rotation(Quat::from_right()),
			..default()
		})
		.id();



	let parent = commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
			material: materials.add(Color::rgb(0., 1., 1.).into()),
			transform: Transform::from_xyz(3., 0., 0.).with_rotation(Quat::from_right()),
			..default()
		})
		.id();
	let child = commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
			material: materials.add(Color::rgb(0., 1., 1.).into()),
			transform: Transform::from_xyz(0., 0., 0.5).with_scale(Vec3::splat(0.5)),
			..default()
		})
		.id();
	commands
		.entity(parent)
		.push_children(&[child]);
}
