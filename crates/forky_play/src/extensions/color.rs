use bevy::prelude::*;
use extend::ext;


#[ext]
pub impl Color {
	fn with_a(mut self, a: f32) -> Self {
		self.set_a(a);
		self
	}
}
