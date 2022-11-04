use bevy::prelude::*;
use forky_core::{math::*, *};
use forky_play::*;
use sweet::*;

sweet! {
	it "works" {
		app::init()
			// .add_plugin(maze::MazePlugin)
			.add_startup_system(spawn_grid)

			.add_plugin(debug::GridPlugin)
			.add_plugin(input::DebugCameraPlugin)
			// .forky_exit_after(2)
			.run();
	}
}



pub fn spawn_grid(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let parent = commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
		material: materials.add(Color::rgb(0., 1., 1.).into()),
		transform: Transform::from_xyz(3., 0., 0.).with_rotation(Quat::from_right()),
		..default()
	}).id();
	let child = commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
		material: materials.add(Color::rgb(0., 1., 1.).into()),
		transform: Transform::from_xyz(0., 0., 0.5)
			.with_scale(Vec3::splat(0.5)),
		..default()
	}).id();
	commands.entity(parent).push_children(&[child]);

}
