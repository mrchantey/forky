use crate::*;
use bevy::prelude::*;
use forky_core::math::*;

pub struct MazePlugin;



impl Plugin for MazePlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.add_startup_system(spawn_grid)
			.forky();
	}
}


pub fn spawn_grid(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let num_rows = 10;
	let num_cols = 10;
	let size = 10.;

	for row in 0..num_rows{
		for col in 0..num_cols{

			let mut x = f(col) / f(num_cols - 1) * 2. - 1.;
			let mut z = f(row) / f(num_rows - 1) * 2. - 1.;
			x = x * size;
			z = z * size;
			// let z: f32 = 0.;
		
			commands.spawn_bundle(PbrBundle {
				mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
				material: materials.add(Color::rgb(0., 1., 1.).into()),
				transform: Transform::from_xyz(x, 0., z),
				..default()
			});

		}
	}

}
