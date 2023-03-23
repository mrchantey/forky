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
	pub fn position(&self, position: f32) -> Vec3 {
		match self {
			Spline::Linear(spline) => {
				bezier3::linear(spline.p0, spline.p1, position)
			}
			Spline::Quadratic(spline) => {
				bezier3::quadratic(spline.p0, spline.p1, spline.p2, position)
			}
			Spline::Cubic(spline) => bezier3::cubic(
				spline.p0, spline.p1, spline.p2, spline.p3, position,
			),
		}
	}

	pub fn tangent(&self, position: f32) -> Vec3 {
		match self {
			Spline::Linear(spline) => {
				bezier3::tangent_linear(spline.p0, spline.p1)
			}
			Spline::Quadratic(spline) => bezier3::tangent_quadratic(
				spline.p0, spline.p1, spline.p2, position,
			),
			Spline::Cubic(spline) => bezier3::tangent_cubic(
				spline.p0, spline.p1, spline.p2, spline.p3, position,
			),
		}
	}
	pub fn acceleration(
		&self,
		position: f32,
		acceleration: Vec3,
	) -> f32 {
		let tangent = self.tangent(position);
		let tangent_force = tangent * acceleration;
		let force_angle = f32::atan2(tangent_force.y, tangent_force.x);
		let force = f32::sin(force_angle);
	
		//chatgpt solution
		// let force_magnitude = acceleration.length();
		// let force_direction = acceleration.normalize();
		// let dot_product = tangent.dot(force_direction);
		// let force = dot_product * force_magnitude;
		force
	}
}
