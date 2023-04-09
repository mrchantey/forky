use super::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct CartSettings {
	pub max_carts: usize,
	pub cart_spawn_interval: Duration,
	pub mesh: Handle<Mesh>,
	pub material: Handle<StandardMaterial>,
}

impl CartSettings {
	pub fn new(
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<StandardMaterial>>,
	) -> Self {
		Self {
			max_carts: 10,
			cart_spawn_interval: Duration::from_secs(1),
			mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
			material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
		}
	}
}


pub fn spawn_cart_settings(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.insert_resource(CartSettings::new(&mut meshes, &mut materials));
	commands.insert_resource(LastCartSpawn(Duration::from_secs(0)));
}
