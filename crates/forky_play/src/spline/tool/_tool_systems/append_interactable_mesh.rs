use crate::spline::tool::*;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;


pub fn append_interactable_mesh(
	mut commands: Commands,
	interaction_settings: Res<InteractionSettings>,
	interaction_resources: Res<InteractionResources>,
	query: Query<Entity, (With<Interactable>, Without<Handle<Mesh>>)>,
) {
	for entity in query.iter() {
		commands.entity(entity).insert((
			materials::RenderBundle {
				material: interaction_resources.inactive_material.clone(),
				mesh: interaction_resources.node_mesh.clone(),
				..Default::default()
			},
			Collider::ball(interaction_settings.node_radius),
		));
	}
}
