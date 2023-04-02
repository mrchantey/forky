use super::*;
use crate::bezier3;
use anyhow::{anyhow, Result};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CubicSpline {
	pub p0: Vec3,
	pub p1: Vec3,
	pub p2: Vec3,
	pub p3: Vec3,
}

impl CubicSpline {
	pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
		Self { p0, p1, p2, p3 }
	}
}

impl SplineType for CubicSpline {
	fn get_points(&self) -> Vec<Vec3> {
		vec![self.p0, self.p1, self.p2, self.p3]
	}

	fn set_point(&mut self, pos: Vec3, index: u32) -> Result<()> {
		match index {
			0 => {
				self.p0 = pos;
				Ok(())
			}
			1 => {
				self.p1 = pos;
				Ok(())
			}
			2 => {
				self.p2 = pos;
				Ok(())
			}
			3 => {
				self.p3 = pos;
				Ok(())
			}
			_ => Err(anyhow!("Invalid index")),
		}
	}

	fn set_first(&mut self, pos: Vec3) { self.p0 = pos }
	fn set_last(&mut self, pos: Vec3) { self.p3 = pos }

	fn position(&self, t: f32) -> Vec3 {
		bezier3::cubic(self.p0, self.p1, self.p2, self.p3, t)
	}

	fn derivative(&self, t: f32) -> Vec3 {
		bezier3::cubic_derivative(self.p0, self.p1, self.p2, self.p3, t)
	}
	fn derivative2(&self, t: f32) -> Vec3 {
		bezier3::cubic_derivative2(self.p0, self.p1, self.p2, self.p3, t)
	}
	fn derivative3(&self, _t: f32) -> Vec3 {
		bezier3::cubic_derivative3(self.p0, self.p1, self.p2, self.p3)
	}
}
