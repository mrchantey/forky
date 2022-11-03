use crate::*;
use bevy::prelude::*;

pub struct MazePlugin;



impl Plugin for MazePlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.add_startup_system(spawn_grid)
			.add_startup_system(base::spawn_camera)
			.forky();

		// app.insert_resource( )
	}
}


pub fn spawn_grid(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let num_rows = 10;
	let num_cols = 10;


	for row in 0..num_rows{
		for col in 0..num_cols{



		}
	}

	for i in 0..10 {
		let x: f32 = i as f32;
		let z: f32 = 0.;

		commands.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
			material: materials.add(Color::rgb(0., 1., 1.).into()),
			transform: Transform::from_xyz(x, 0., z),
			..default()
		});
	}
}
