use crate::base::*;
use bevy::prelude::*;
use forky_core::math::*;

pub struct Kaleid;

pub struct KaleidNode {}

impl Plugin for Kaleid {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, Self::spawn_meshes)
			.add_systems(Update, Self::rotate_cube);
	}
}


impl Kaleid {
	pub fn spawn_meshes(
		mut commands: Commands,
		mut meshes: ResMut<Assets<Mesh>>,
		mut materials: ResMut<Assets<StandardMaterial>>,
	) {
		let mesh = meshes.add(Mesh::from(Cuboid::default()));
		let material = materials.add(Color::rgb(0., 1., 1.));
		commands
			.spawn(PbrBundle {
				mesh,
				material,
				transform: Transform::from_xyz(0., 1., 0.),
				..default()
			})
			.insert(CompanionCube);
	}
	pub fn rotate_cube(
		time: Res<Time>,
		mut query: Query<&mut Transform, With<CompanionCube>>,
	) {
		for mut transform in &mut query {
			transform.rotate_x(TAU * 0.1 * time.delta_seconds());
		}
	}
}
