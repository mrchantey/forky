use crate::*;
use anyhow::Result;
use bevy::prelude::*;
use forky_core::prelude::*;
// cross product visualizer https://www.geogebra.org/m/psMTGDgc

pub trait SplineType {
	fn set_point(&mut self, pos: Vec3, index: u32) -> Result<()>;
	fn first(&self) -> Vec3;
	fn last(&self) -> Vec3;
	fn set_first(&mut self, pos: Vec3);
	fn set_last(&mut self, pos: Vec3);
	fn get_points(&self) -> Vec<Vec3>;
	fn position(&self, t: f32) -> Vec3;
	fn derivative(&self, t: f32) -> Vec3;
	fn derivative2(&self, t: f32) -> Vec3;
	fn derivative3(&self, t: f32) -> Vec3;
	fn tangent(&self, t: f32) -> Vec3 { self.derivative(t).normalize_or_zero() }

	fn normal(&self, t: f32) -> Vec3 {
		let dt = 0.01;
		let is_at_end = t + dt > 1.;
		let tangent1 = tern!(is_at_end; self.tangent(t - dt); self.tangent(t));
		let tangent2 =
			tern!(is_at_end; self.tangent(t);			self.tangent(t + dt));
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
	fn normal_up(&self, t: f32, up: Vec3) -> Vec3 {
		let mut tangent = self.tangent(t);
		// account for vertical tangents
		if tangent.abs() == Vec3::UP {
			let dt = 0.01;
			let is_at_end = t + dt > 1.;
			tangent =
				tern!(is_at_end; self.tangent(t - dt); self.tangent(t + dt));
		}
		if tangent.abs() == Vec3::UP {
			tangent.x += 0.1;
			tangent = tangent.normalize();
		}


		// account for inverted normals
		let binormal = up.cross(tangent);
		let normal = tangent.cross(binormal);

		if normal.y >= 0. {
			normal.normalize()
		// up.cross(tangent).normalize()
		} else {
			Vec3::RIGHT
			// tangent.cross((-up).cross(tangent)).normalize()
			// -normal
			// -up.cross(tangent).normalize()
		}
		// tangent.cross(normal)

		// up.cross(tangent)
	}

	fn acceleration(&self, position: f32, acceleration: Vec3) -> f32 {
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


	fn path(&self, num_nodes: usize) -> Vec<Vec3> {
		let mut points = Vec::with_capacity(num_nodes);
		let segments = num_nodes.max(1);
		let step = 1.0 / segments as f32;

		for i in 0..=segments {
			let t = i as f32 * step;
			let point = self.position(t);
			points.push(point);
		}

		points
	}

	fn total_length(&self, subdivisions: usize) -> f32 {
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

	fn get_lengths(&self, subdivisions: usize) -> Vec<f32> {
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


// pub struct SplineLengthIterator<'a> {
// 	pub spline: &'a Spline,
// 	pub divisions: usize,
// 	pub delta_t: f32,
// 	pub t: f32,
// 	pub last: Vec3,
// 	pub len: f32,
// }

// impl<'a> SplineLengthIterator<'a> {
// 	pub fn new(spline: &'a Spline, subdivisions: usize) -> Self {
// 		SplineLengthIterator::<'a> {
// 			spline,
// 			divisions: subdivisions + 2,
// 			delta_t: 1. / (subdivisions + 1) as f32,
// 			t: 0.,
// 			last: spline.position(0.),
// 			len: 0.,
// 		}
// 	}
// }
