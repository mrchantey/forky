use crate::*;
use bevy::prelude::*;
use derive_deref::{Deref, DerefMut};


#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Spline {
	Linear(LinearSpline),
	Quadratic(QuadraticSpline),
	Cubic(CubicSpline),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LinearSpline {
	pub p0: Vec3,
	pub p1: Vec3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QuadraticSpline {
	pub p0: Vec3,
	pub p1: Vec3,
	pub p2: Vec3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

	pub fn total_length(&self, subdivisions: usize) -> f32 {
		let mut len = 0.;
		let mut last = self.position(0.);
		let divisions = subdivisions + 2;
		let delta_t = 1. / (divisions - 1) as f32;
		for i in 1..divisions {
			let t = i as f32 * delta_t;
			let next = self.position(t);
			len += (next - last).length();
			last = next;
		}
		len
	}

	pub fn get_lengths(&self, subdivisions: usize) -> Vec<f32> {
		let mut len = 0.;
		let mut last = self.position(0.);
		let divisions = subdivisions + 2;
		let delta_t = 1. / (divisions - 1) as f32;
		let mut lengths = Vec::with_capacity(divisions);
		lengths.push(0.);
		for i in 1..divisions {
			let t = i as f32 * delta_t;
			let next = self.position(t);
			len += (next - last).length();
			last = next;
			lengths.push(len);
		}
		lengths
	}
}


pub struct SplineLengthIterator<'a> {
	spline: &'a Spline,
	divisions: usize,
	delta_t: f32,
	t: f32,
	last: Vec3,
	len: f32,
}

impl<'a> SplineLengthIterator<'a> {
	pub fn new(spline: &'a Spline, subdivisions: usize) -> Self {
		let total_len = spline.total_length(subdivisions);
		SplineLengthIterator::<'a> {
			spline,
			divisions: subdivisions + 2,
			delta_t: 1. / (subdivisions + 1) as f32,
			t: 0.,
			last: spline.position(0.),
			len: 0.,
		}
	}
}
