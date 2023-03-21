use bevy::prelude::*;
use crate::bezier;





pub fn bezier_path_3d(p0:Vec3,p1:Vec3,p2:Vec3,p3:Vec3,len:u32)-> Vec<Vec3>{
	let mut points = Vec::new();
	let segments = len.max(1);
	let step = 1.0 / segments as f32;

	for i in 0..=segments {
			let t = i as f32 * step;
			let point = bezier::cubic3d(p0, p1, p2, p3, t);
			points.push(point);
	}

	points
}