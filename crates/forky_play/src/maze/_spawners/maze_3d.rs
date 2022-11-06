use crate::maze::*;
use crate::*;
use bevy::{prelude::*, transform};
use forky_core::{math::*, *};

use super::*;

pub struct RespawnEvent {
	pub num_cols: usize,
	pub num_rows: usize,
	pub cell_width: f32,
	pub wall_width: f32,
	pub wall_height: f32,
}

pub struct DespawnEvent;

#[derive(Component)]
pub struct MazeItemTag;

impl Default for RespawnEvent {
	fn default() -> Self {
		Self {
			num_cols: 6,
			num_rows: 6,
			cell_width: 2.,
			wall_width: 0.2,
			wall_height: 0.5,
		}
	}
}

#[derive(Component, Default)]
pub struct MazeGame {
	score: usize,
}

pub fn respawn(
	mut spawn_event: EventWriter<RespawnEvent>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	query: Query<(Entity, &MazeGame)>,
) {
	if query.is_empty() {
		commands
			.spawn()
			.insert(MazeGame::default())
			.insert(MazeItemTag);

		let e = RespawnEvent::default();
		spawn_event.send(e);
	}
}

pub fn despawn(
	mut despawn_event: EventReader<maze_3d::DespawnEvent>,
	mut spawn_event: EventReader<maze_3d::DespawnEvent>,
	mut commands: Commands,
	q_items: Query<Entity, With<MazeItemTag>>,
) {
	for e in despawn_event.iter() {
		for entity in q_items.iter() {
			commands.entity(entity).despawn_recursive();
		}
	}
}
