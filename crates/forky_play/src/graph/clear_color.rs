use bevy::prelude::*;

#[derive(Resource, Copy, Clone, Default)]
pub struct CustomClearColor {
	pub r: f64,
	pub g: f64,
	pub b: f64,
	pub a: f64,
}
