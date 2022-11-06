use crate::maze::*;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math::*, *};

#[derive(Component)]
pub struct MazeBoardTag;

pub fn respawn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut spawn_event: EventReader<RespawnEvent>,
) {
	for e in spawn_event.iter() {
		let mut maze = RectMazeSpatial::new(
			e.num_cols,
			e.num_rows,
			e.cell_width,
			e.wall_width,
			e.wall_height,
		);
		maze.rect_maze.depth_first_backtrack(|f| {});

		let walls = maze_wall::spawn_all(
			&mut commands,
			&mut meshes,
			&mut materials,
			&maze,
		);
		let floor =
			floor::spawn(&mut commands, &mut meshes, &mut materials, &maze);
		let controller = board_joint::force_spawn(
			&mut commands,
			&mut meshes,
			&mut materials,
		);
		commands
			.entity(controller)
			.insert(MazeItemTag)
			.insert(MazeBoardTag)
			.push_children(&[walls, floor]);
	}
}
