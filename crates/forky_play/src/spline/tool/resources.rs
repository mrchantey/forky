use super::*;
use crate::{spline::graph::SplineNode, *};
use bevy::prelude::{*};
use bevy_rapier3d::prelude::Collider;



#[derive(Resource)]
pub struct InteractionSettings {
	pub node_radius: f32,
	pub height_delta: f32,
	pub highlight_color: Color,
	pub select_color: Color,
	pub select_primary_color: Color,
	pub inactive_color: Color,
}
#[derive(Resource)]
pub struct InteractionResources {
	pub node_mesh: Handle<Mesh>,
	pub highlight_material: Handle<materials::UnlitMaterial>,
	pub select_material: Handle<materials::UnlitMaterial>,
	pub select_primary_material: Handle<materials::UnlitMaterial>,
	pub inactive_material: Handle<materials::UnlitMaterial>,
}

impl Default for InteractionSettings {
	fn default() -> Self {
		Self {
			inactive_color: Color::rgb(0., 0.8, 0.),
			highlight_color: Color::rgb(0., 1., 0.),
			select_color: Color::rgb(0., 0., 0.8),
			select_primary_color: Color::rgb(0., 0., 1.),
			node_radius: 0.1,
			height_delta: 0.1,
		}
	}
}

pub const SELECT_BUTTON: MouseButton = MouseButton::Left;
// pub const DESELECT_BUTTON: MouseButton = MouseButton::Right;
pub fn spawn_resources(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<materials::UnlitMaterial>>,
	settings: Res<InteractionSettings>,
) {
	commands.insert_resource(InteractionResources {
		node_mesh: meshes.add(Mesh::from(shape::UVSphere {
			radius: settings.node_radius,
			sectors: 8,
			stacks: 8,
		})),
		highlight_material: materials.add(settings.highlight_color.into()),
		select_material: materials.add(settings.select_color.into()),
		select_primary_material: materials
			.add(settings.select_primary_color.into()),
		inactive_material: materials.add(settings.inactive_color.into()),
	});
}

pub fn append_node_meshes(
	mut commands: Commands,
	settings: Res<InteractionSettings>,
	resources: Res<InteractionResources>,
	query: Query<Entity, (With<SplineNode>, Without<Handle<Mesh>>)>,
) {
	for entity in query.iter() {
		commands
			.entity(entity)
			.insert(materials::RenderBundle {
				material: resources.inactive_material.clone(),
				mesh: resources.node_mesh.clone(),
				..Default::default()
			})
			.insert(Collider::ball(settings.node_radius))
			.insert(Interactable);
	}
}
