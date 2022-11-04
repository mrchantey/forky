use crate::{maze::mesh_shape, *};
use bevy::prelude::*;
use forky_core::{math::*, *};

pub fn spawn_all(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	walls: &Vec<(Transform, Vec<Transform>)>,
	wall_height: f32,
) -> Entity {
	let walls: Vec<Entity> = walls
		.iter()
		.map(|(parent, children)| {
			spawn(commands, meshes, materials, parent, children)
		})
		.collect();


	commands
		.spawn_bundle(SpatialBundle {
			transform: Transform::from_xyz(0., wall_height / 2., 0.)
				.with_scale_y(wall_height),
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
					material: materials.add(Color::rgb(1., 1., 1.).into()),
					..default()
				})
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
