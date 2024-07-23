use crate::*;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct InteractionSettings {
	pub node_radius: f32,
	pub height_delta: f32,
	pub highlight_color: Color,
	pub select_color: Color,
	pub select_primary_color: Color,
	pub inactive_color: Color,
	pub intersect_normal: Vec3,
}

impl Default for InteractionSettings {
	fn default() -> Self {
		Self {
			inactive_color: Color::rgb(0., 0.8, 0.),
			highlight_color: Color::rgb(0., 1., 0.),
			select_color: Color::rgb(0., 0., 0.8),
			select_primary_color: Color::rgb(0., 0., 1.),
			node_radius: 0.3,
			height_delta: 0.1,
			intersect_normal: Vec3::UP,
		}
	}
}

#[derive(Resource)]
pub struct InteractionResources {
	pub node_mesh: Handle<Mesh>,
	pub highlight_material: Handle<materials::UnlitMaterial>,
	pub select_material: Handle<materials::UnlitMaterial>,
	pub select_primary_material: Handle<materials::UnlitMaterial>,
	pub inactive_material: Handle<materials::UnlitMaterial>,
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
		node_mesh: meshes.add(Mesh::from(Sphere {
			radius: settings.node_radius,
		})),
		highlight_material: materials.add(settings.highlight_color),
		select_material: materials.add(settings.select_color),
		select_primary_material: materials.add(settings.select_primary_color),
		inactive_material: materials.add(settings.inactive_color),
	});
}
