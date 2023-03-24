use crate::*;
use bevy::prelude::*;
use derive_deref::{Deref, DerefMut};


#[derive(Component)]
pub enum Spline {
	Linear(LinearSpline),
	Quadratic(QuadraticSpline),
	Cubic(CubicSpline),
}

pub struct LinearSpline {
	pub p0: Vec3,
	pub p1: Vec3,
}
pub struct QuadraticSpline {
	pub p0: Vec3,
	pub p1: Vec3,
	pub p2: Vec3,
}
pub struct CubicSpline {
	pub p0: Vec3,
	pub p1: Vec3,
	pub p2: Vec3,
	pub p3: Vec3,
}


impl Spline {
	pub fn position(&self, t: f32) -> Vec3 {
		match self {
			Spline::Linear(spline) => bezier3::linear(spline.p0, spline.p1, t),
			Spline::Quadratic(spline) => {
				bezier3::quadratic(spline.p0, spline.p1, spline.p2, t)
			}
			Spline::Cubic(spline) => {
				bezier3::cubic(spline.p0, spline.p1, spline.p2, spline.p3, t)
			}
		}
	}

	pub fn tangent(&self, t: f32) -> Vec3 {
		match self {
			Spline::Linear(spline) => {
				bezier3::tangent_linear(spline.p0, spline.p1)
			}
			Spline::Quadratic(spline) => {
				bezier3::tangent_quadratic(spline.p0, spline.p1, spline.p2, t)
			}
			Spline::Cubic(spline) => bezier3::tangent_cubic(
				spline.p0, spline.p1, spline.p2, spline.p3, t,
			),
		}
	}
	pub fn acceleration(&self, position: f32, acceleration: Vec3) -> f32 {
		if (acceleration.length() == 0.) {
			return 0.;
		}
		//find scalar projection of acceleration vector onto tangent
		let tangent = self.tangent(position);
		let acc_dir = acceleration.normalize();
		let dot = tangent.dot(acc_dir);
		let force = dot * acceleration.length();
		force
	}


	pub fn path(&self, num_nodes: u32) -> Vec<Vec3> {
		let mut points = Vec::new();
		let segments = num_nodes.max(1);
		let step = 1.0 / segments as f32;

		for i in 0..=segments {
			let t = i as f32 * step;
			let point = self.position(t);
			points.push(point);
		}

		points
	}
}
