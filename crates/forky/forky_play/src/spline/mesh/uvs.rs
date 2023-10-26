use crate::spline::*;

use bevy::prelude::*;

pub fn spline_to_uv(
	spline: &Spline,
	edge_loop_len: usize,
	subdivisions: usize,
) -> Vec<Vec2> {
	let divisions = subdivisions + 2;

	let capacity = edge_loop_len * divisions;
	let mut uv = Vec::with_capacity(capacity);

	let lengths = spline.get_lengths(subdivisions);
	let total_len = lengths.last().unwrap();

	for iv in 0..divisions {
		let v = lengths[iv] / total_len;
		// let divisions = divisions as f32;
		for iu in 0..edge_loop_len {
			let u = iu as f32 / (edge_loop_len - 1) as f32;
			uv.push(Vec2::new(u, v));
		}
	}
	uv
}
