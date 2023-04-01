use super::*;
use crate::spline::Spline;

use bevy::prelude::*;
use bevy::render::mesh;
use wgpu::PrimitiveTopology;


pub fn create_spline_mesh(spline: &Spline,subdivisions:usize) -> Mesh {
	let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

	let edge_loop = rect_edge_loop(1., 0.2);
	let vertices = spline_to_vertices(&spline, &edge_loop, subdivisions);
	let triangles = spline_to_triangles(edge_loop.len(), subdivisions);
	let uvs = spline_to_uv(&spline, edge_loop.len(), subdivisions);
	let num_verts = vertices.len();

	mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
	mesh.insert_attribute(
		Mesh::ATTRIBUTE_NORMAL,
		vec![[0., 1., 0.]; num_verts],
	);
	mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
	mesh.set_indices(Some(mesh::Indices::U32(triangles)));

	mesh
}