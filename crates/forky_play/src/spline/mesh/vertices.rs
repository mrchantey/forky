use crate::spline::Spline;
use bevy::prelude::*;



/// Create Vertices
///
/// # Arguments
///
/// * `spline` - The spline used
/// * `edge_loop` - The edge loop used along the spline
/// * `subdivisions` - 0 means a straight line, 1 has one break, etc
///
pub fn spline_to_vertices(
	spline: &Spline,
	edge_loop: &Vec<Vec2>,
	subdivisions: usize,
) -> Vec<Vec3> {
	let divisions = subdivisions + 2;
	let capacity = edge_loop.len() * divisions;
	let mut vertices = Vec::with_capacity(capacity);

	let delta_t = 1. / (divisions - 1) as f32;

	for i in 0..divisions {
		let t = i as f32 * delta_t;
		let pos = spline.position(t);
		let tangent = spline.tangent(t);
		let normal = spline.normal(t);
		let binormal = normal.cross(tangent).normalize();
		for point in edge_loop.iter() {
			let vertex = pos + normal * point.x + binormal * point.y;
			vertices.push(vertex);
		}
	}
	vertices
}


pub fn vertex_divisions(vertices: &Vec<Vec3>, edge_loop: &Vec<Vec2>) -> usize {
	vertices.len() / edge_loop.len()
}
