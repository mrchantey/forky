use bevy::prelude::*;
use super::*;

pub fn linear(p0: Vec3, p1: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::linear(p0.x, p1.x, t),
		bezier::linear(p0.y, p1.y, t),
		bezier::linear(p0.z, p1.z, t),
	)
}

pub fn quadratic(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::quadratic(p0.x, p1.x, p2.x, t),
		bezier::quadratic(p0.y, p1.y, p2.y, t),
		bezier::quadratic(p0.z, p1.z, p2.z, t),
	)
}

pub fn cubic(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::cubic(p0.x, p1.x, p2.x, p3.x, t),
		bezier::cubic(p0.y, p1.y, p2.y, p3.y, t),
		bezier::cubic(p0.z, p1.z, p2.z, p3.z, t),
	)
}

pub fn tangent_linear(p0: Vec3, p1: Vec3) -> Vec3 {
	Vec3::new(
		bezier::tangent_linear(p0.x, p1.x),
		bezier::tangent_linear(p0.y, p1.y),
		bezier::tangent_linear(p0.z, p1.z),
	).normalize()
}

pub fn tangent_quadratic(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::tangent_quadratic(p0.x, p1.x, p2.x, t),
		bezier::tangent_quadratic(p0.y, p1.y, p2.y, t),
		bezier::tangent_quadratic(p0.z, p1.z, p2.z, t),
	).normalize()
}

pub fn tangent_cubic(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
	Vec3::new(
		bezier::tangent_cubic(p0.x, p1.x, p2.x, p3.x, t),
		bezier::tangent_cubic(p0.y, p1.y, p2.y, p3.y, t),
		bezier::tangent_cubic(p0.z, p1.z, p2.z, p3.z, t),
	).normalize()
}


pub fn path(p0:Vec3,p1:Vec3,p2:Vec3,p3:Vec3,len:u32)-> Vec<Vec3>{
	let mut points = Vec::new();
	let segments = len.max(1);
	let step = 1.0 / segments as f32;

	for i in 0..=segments {
			let t = i as f32 * step;
			let point = bezier3::cubic(p0, p1, p2, p3, t);
			points.push(point);
	}

	points
}