use super::*;
use crate::*;
use bevy::prelude::*;
pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_systems(Update, mouse_controller)
			.add_systems(Update, keyboard_controller)
			.__();
	}
}
