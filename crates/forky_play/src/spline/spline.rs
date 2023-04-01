use crate::*;
use bevy::prelude::*;
use forky_core::*;



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
	pub fn set_first(&mut self, p: Vec3) {
		match self {
			Spline::Linear(spline) => spline.p0 = p,
			Spline::Quadratic(spline) => spline.p0 = p,
			Spline::Cubic(spline) => spline.p0 = p,
		}
	}
	pub fn set_last(&mut self, p: Vec3) {
		match self {
			Spline::Linear(spline) => spline.p1 = p,
			Spline::Quadratic(spline) => spline.p2 = p,
			Spline::Cubic(spline) => spline.p3 = p,
		}
	}

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

	pub fn normal(&self, t: f32) -> Vec3 {
		let dt = 0.01;
		let is_at_end = t + dt > 1.;
		let tangent1 = tern!(is_at_end; self.tangent(t - dt); self.tangent(t));
		let tangent2 = tern!(is_at_end; self.tangent(t);			self.tangent(t + dt));
		let normal = tangent1.cross(tangent2).normalize_or_zero();

		if normal == Vec3::ZERO {
			tern!(tangent1.abs() == Vec3::UP
				;tangent1.cross(Vec3::RIGHT)
				;tangent1.cross(Vec3::UP)
			)
		} else {
			normal
		}
	}
	// public static (Matrix4x4 WorldToLocal, Matrix4x4 LocalToWorld) PointsToMatrix(Vector3 a, Vector3 b, Vector3 c)
	// {
	// 	Vector3 dirY = (b - a).normalized;
	// 	Vector3 ab = b - a;
	// 	Vector3 ac = c - a;
	// 	float thetaBAC = AngleBetween(ab, ac);
	// 	float ad = Mathf.Cos(thetaBAC) * ac.magnitude;
	// 	Vector3 d = a + dirY * ad;
	// 	Vector3 dc = c - d;
	// 	Vector3 dirX = dc.normalized;
	// 	Vector3 dirZ = Vector3.Cross(dirX, dirY).normalized;
	// 	var matrix = new Matrix4x4(dirX, dirY, dirZ, new Vector4(a.x, a.y, a.z, 1));
	// 	return (matrix.inverse, matrix);
	// }


	pub fn acceleration(&self, position: f32, acceleration: Vec3) -> f32 {
		if acceleration.length() == 0. {
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
	pub spline: &'a Spline,
	pub divisions: usize,
	pub delta_t: f32,
	pub t: f32,
	pub last: Vec3,
	pub len: f32,
}

impl<'a> SplineLengthIterator<'a> {
	pub fn new(spline: &'a Spline, subdivisions: usize) -> Self {
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
