use super::*;
use crate::maze::*;
use crate::*;
use bevy::{prelude::*, transform};
use forky_core::{math::*, *};

pub fn respawn(
	game: ResMut<MazeGame>,
	mut spawn_event: EventWriter<RespawnEvent>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	query: Query<&MazeItemTag>,
) {
	if query.is_empty() {
		commands.spawn().insert(MazeItemTag);

		let level = game.score + 1;
		spawn_event.send(RespawnEvent {
			level,
			num_cols: level + 1,
			num_rows: level + 1,
			..default()
		});
	}
}

pub fn despawn(
	mut game: ResMut<MazeGame>,
	mut despawn_event: EventReader<DespawnEvent>,
	mut spawn_event: EventReader<DespawnEvent>,
	mut commands: Commands,
	q_items: Query<Entity, With<MazeItemTag>>,
) {
	for _ in despawn_event.iter() {
		for entity in q_items.iter() {
			commands.entity(entity).despawn_recursive();
		}
	}
}
