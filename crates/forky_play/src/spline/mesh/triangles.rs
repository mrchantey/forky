use bevy::prelude::*;


pub fn spline_to_triangles(
	edge_loop_len: usize,
	subdivisions: usize,
) -> Vec<u32> {
	let segments = subdivisions + 1;
	let capacity = edge_loop_len * segments * 6;
	let mut triangles = Vec::with_capacity(capacity);
	for si in 0..segments {
		let wrap_index = edge_loop_len + si * edge_loop_len;
		for li in 0..edge_loop_len {
			let i0 = si * edge_loop_len + li;
			let mut i1 = i0 + 1;
			if i1 >= wrap_index {
				i1 -= edge_loop_len;
			}
			let i2 = i0 + edge_loop_len;
			let i3 = i1 + edge_loop_len;
			triangles.push(i0 as u32);
			triangles.push(i1 as u32);
			triangles.push(i2 as u32);
			triangles.push(i1 as u32);
			triangles.push(i3 as u32);
			triangles.push(i2 as u32);
		}
	}
	triangles
}
