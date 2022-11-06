use super::*;
use crate::maze::*;
use crate::*;
use bevy::{prelude::*, transform};
use forky_core::{math::*, *};

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
