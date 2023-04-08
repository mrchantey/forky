use super::*;
use crate::*;
use bevy::prelude::*;
pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_system(mouse_controller)
			.add_system(keyboard_controller)
			.__();
	}
}
