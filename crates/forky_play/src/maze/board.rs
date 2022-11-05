use super::*;
use crate::{maze::mesh_shape, *};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math::*, *};

#[derive(Component)]
pub struct MazeBoard;


pub fn spawn(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	maze: &RectMazeSpatial,
) -> Entity {
	let walls = maze_wall::spawn_all(commands, meshes, materials, &maze);
	let floor = floor::spawn(commands, meshes, materials, &maze);

	let controller = board_joint::spawn(commands, meshes, materials);

	// commands.entity(controller).push_children(&[walls]);
	commands.entity(controller).push_children(&[walls, floor]);
	controller
}
