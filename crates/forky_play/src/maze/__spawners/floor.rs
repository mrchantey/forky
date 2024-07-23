use crate::maze::rect_maze::RectMazeSpatial;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;


pub fn spawn(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	maze: &RectMazeSpatial,
) -> Entity {
	let floor_h = maze.wall_height / 2.;
	let floor_w = maze.cols() as f32 * maze.cell_width + maze.wall_width;
	let floor_d = maze.rows() as f32 * maze.cell_width + maze.wall_width;
	commands
		.spawn(PbrBundle {
			transform: Transform::from_position_y(-floor_h / 2.)
				.with_scale_xyz(floor_w, floor_h, floor_d),
			mesh: meshes.add(Mesh::from(Cuboid::default())),
			material: materials.from_white(),
			..default()
		})
		.insert(Collider::cuboid(0.5, 0.5, 0.5))
		.insert(ColliderMassProperties::Density(0.))
		.id()
}
