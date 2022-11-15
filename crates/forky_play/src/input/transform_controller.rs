use bevy::prelude::*;
//https://bevy-cheatbook.github.io/input/keyboard.html

#[derive(Component)]
pub struct TransformController {
	pub rotate_speed: f32,
	pub translate_speed: f32,
}

impl Default for TransformController {
	fn default() -> Self {
		TransformController {
			translate_speed: 10.,
			rotate_speed: 20.,
		}
	}
}
