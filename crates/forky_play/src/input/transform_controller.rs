use bevy::prelude::*;
//https://bevy-cheatbook.github.io/input/keyboard.html

#[derive(Component)]
pub struct ActiveTransformController;

#[derive(Component, Clone)]
pub struct TransformController {
	pub rotate_speed: f32,
	pub translate_speed: f32,
	pub local_axis: bool,
	pub allow_rotation: bool,
}

impl Default for TransformController {
	fn default() -> Self {
		TransformController {
			translate_speed: 10.,
			rotate_speed: 20.,
			local_axis: false,
			allow_rotation: true,
		}
	}
}

impl TransformController {
	pub fn with_local_axis(&mut self) -> &mut Self {
		self.local_axis = true;
		self
	}
	pub fn with_rotation_disabled(&mut self) -> &mut Self {
		self.allow_rotation = false;
		self
	}
}
