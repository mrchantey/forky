use crate::{maze::mesh_shape, *};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math::*, *};

use super::RectMazeSpatial;



pub fn spawn(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	maze: &RectMazeSpatial,
) -> Entity {
	let h_cell_width = maze.cell_width / 2.;
	let diameter = h_cell_width * 0.5;
	commands
		.spawn_bundle(PbrBundle {
			transform: Transform::from_xyz(
				-h_cell_width,
				h_cell_width,
				-h_cell_width,
			)
			.with_scale_value(diameter),
			mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
			material: materials.add(Color::rgb(1., 1., 1.).into()),
			..default()
		})
		.insert(RigidBody::Dynamic)
		.insert(Collider::ball(1.))
		.insert(Restitution {
			coefficient: 0.5,
			combine_rule: CoefficientCombineRule::Average,
		})
		.insert(Friction {
			coefficient: 0.,
			combine_rule: CoefficientCombineRule::Average,
		})
		.insert(ColliderDebugColor(Color::RED))
		.id()
}
