use crate::maze::*;
use bevy::prelude::*;

pub fn respawn(
	game: ResMut<MazeGame>,
	mut spawn_event: EventWriter<RespawnEvent>,
	mut commands: Commands,
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
	mut despawn_event: EventReader<DespawnEvent>,
	mut commands: Commands,
	q_items: Query<Entity, With<MazeItemTag>>,
) {
	for _ in despawn_event.iter() {
		for entity in q_items.iter() {
			commands.entity(entity).despawn_recursive();
		}
	}
}
