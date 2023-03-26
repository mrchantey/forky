use super::*;
use crate::materials::UvMaterial;
use crate::spline::Spline;
use crate::*;
use bevy::prelude::*;
use bevy::render::mesh;
use wgpu::PrimitiveTopology;

pub fn append_spline_mesh(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<UvMaterial>>,
	query: Query<(Entity, &Spline), Without<Handle<Mesh>>>,
) {
	for (entity, spline) in query.iter() {
		let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

		let edge_loop = rect_edge_loop(1., 0.2);
		let vertices = spline_to_vertices(&spline, &edge_loop, 10);
		let triangles = spline_to_triangles(edge_loop.len(), 10);
		let uvs = spline_to_uv(&spline, edge_loop.len(), 10);
		let num_verts = vertices.len();

		mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
		mesh.insert_attribute(
			Mesh::ATTRIBUTE_NORMAL,
			vec![[0., 1., 0.]; num_verts],
		);
		mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
		mesh.set_indices(Some(mesh::Indices::U32(triangles)));

		// mesh.insert_attribute(
		// 	Mesh::ATTRIBUTE_POSITION,
		// 	vec![[0., 0., 0.], [1., 2., 1.], [2., 0., 0.]],
		// );

		// mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
		// mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);
		// mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));

		commands.entity(entity).insert(MaterialMeshBundle {
			mesh: meshes.add(mesh),
			material: materials.add(crate::materials::UvMaterial::default()),
			// material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
			..default()
		});
	}
}
