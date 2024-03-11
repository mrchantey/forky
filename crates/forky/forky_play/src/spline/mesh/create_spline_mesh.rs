use super::*;
use crate::spline::Spline;
use bevy::prelude::*;
use bevy::render::mesh;
use bevy::render::render_asset::RenderAssetUsages;
use wgpu::PrimitiveTopology;


pub fn create_spline_mesh(spline: &Spline, subdivisions: usize) -> Mesh {
	let mut mesh = Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::default(),
	);

	let edge_loop = rect_edge_loop(0.1, 0.02);
	let vertices = spline_to_vertices(&spline, &edge_loop, subdivisions);
	let triangles = spline_to_triangles(edge_loop.len(), subdivisions);
	let uvs = spline_to_uv(&spline, edge_loop.len(), subdivisions);
	let num_verts = vertices.len();

	mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
	mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![
		[0., 1., 0.];
		num_verts
	]);
	mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
	mesh.insert_indices(mesh::Indices::U32(triangles));

	mesh
}
