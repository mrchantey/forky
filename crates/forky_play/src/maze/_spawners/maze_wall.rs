use crate::maze::*;
use crate::{maze::mesh_shape, *};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math::*, *};


pub fn spawn_all(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	maze: &RectMazeSpatial,
) -> Entity {
	let walls: Vec<Entity> = maze
		.transforms()
		.iter()
		.map(|(parent, children)| {
			spawn(commands, meshes, materials, parent, children)
		})
		.collect();

	commands
		.spawn_bundle(SpatialBundle {
			transform: Transform::from_xyz(0., maze.wall_height / 2., 0.)
				.with_scale_y(maze.wall_height),
			..default()
		})
		.push_children(&walls)
		.id()
}


pub fn spawn(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	parent: &Transform,
	children: &Vec<Transform>,
) -> Entity {
	let children: Vec<Entity> = children
		.iter()
		.map(|transform| {
			commands
				.spawn_bundle(PbrBundle {
					transform: *transform,
					mesh: meshes.add(Mesh::from(shape::Cube::default())),
					material: materials.from_white(),
					..default()
				})
				.insert(Collider::cuboid(0.5, 0.5, 0.5))
				.insert(ColliderMassProperties::Density(0.))
				.id()
		})
		.collect();


	let parent = commands
		.spawn_bundle(SpatialBundle {
			transform: *parent,
			..default()
		})
		.push_children(&children)
		.id();

	parent
}
