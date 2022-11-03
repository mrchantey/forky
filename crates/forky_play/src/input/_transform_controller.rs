use bevy::prelude::*;
use forky_core::math::TAU;
//https://bevy-cheatbook.github.io/input/keyboard.html
use super::*;
use crate::*;

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
