use std::cmp;

use bevy::prelude::*;
use extend::ext;
use forky_core::*;

use crate::utility;

#[ext]
pub impl Assets<StandardMaterial> {
	//TODO forward faces back
	fn from_white(&mut self) -> Handle<StandardMaterial> {
		self.add(Color::WHITE.into())
	}
	fn from_color(&mut self, color: Color) -> Handle<StandardMaterial> {
		self.add(color.into())
	}
	fn from_rgb(&mut self, r: f32, g: f32, b: f32) -> Handle<StandardMaterial> {
		self.add(Color::rgb(r, g, b).into())
	}
}
