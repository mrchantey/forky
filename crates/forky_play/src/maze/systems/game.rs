use crate::maze::*;
use bevy::prelude::*;

pub fn increment_score(
	mut event: EventReader<DespawnEvent>,
	mut game: ResMut<MazeGame>,
) {
	for e in event.iter() {
		if *e == DespawnEvent::Success {
			game.score += 1;
		}
	}
}
