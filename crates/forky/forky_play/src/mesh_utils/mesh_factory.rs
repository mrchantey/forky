use crate::*;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::mesh::{
	self,
};
use bevy::render::render_asset::RenderAssetUsages;
/// set up a simple 3D scene
pub fn create_mesh(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let vertices = [
		([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
		([1.0, 2.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
		([2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
	];

	let indices = mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]);

	let mut positions = Vec::new();
	let mut normals = Vec::new();
	let mut uvs = Vec::new();
	for (position, normal, uv) in vertices.iter() {
		positions.push(*position);
		normals.push(*normal);
		uvs.push(*uv);
	}

	let mut mesh = Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::default(),
	);
	mesh.insert_indices(indices);
	mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
	mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
	mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

	// add entities to the world
	// plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(mesh),
		material: materials.from_white(),
		..default()
	});
	// light

	// camera
	// commands.spawn(Camera3dBundle {
	// 	transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
	// 	..default()
	// });
}
