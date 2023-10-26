use bevy::prelude::*;
//https://bevy-cheatbook.github.io/input/keyboard.html
use crate::maze::*;

pub fn reset_game_on_key(
	mut game: ResMut<MazeGame>,
	mut despawn_event: EventWriter<DespawnEvent>,
	keys: Res<Input<KeyCode>>,
) {
	if keys.any_just_pressed([KeyCode::Space]) {
		game.score = 0;
		despawn_event.send(DespawnEvent::Reset);
	}
}
