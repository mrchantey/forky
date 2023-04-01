use super::*;

use bevy::prelude::*;


#[derive(Component)]
pub struct Hovered;
#[derive(Component)]
pub struct Highlighted;
#[derive(Component)]
pub struct Selected;
#[derive(Component)]
pub struct PrimaryInteracted;
#[derive(Component)]
pub struct Interactable;

//TODO not run every frame
#[rustfmt::skip]
pub fn on_interact_state_change(
	mut commands: Commands,
	materials: Res<InteractionResources>,
	inactive: Query<Entity, (With<Interactable>,Without<Selected>,Without<Highlighted>)>,
	highlight: Query<Entity, (With<Highlighted>,Without<Selected>)>,
	selected: Query<Entity, (With<Selected>,Without<PrimaryInteracted>)>,
	selcted_highlighted: Query<Entity, (With<Selected>,With<PrimaryInteracted>)>,
) {
	for entity in inactive.iter() {
		commands
		.entity(entity)
		.insert(materials.inactive_material.clone());
	}
	for entity in highlight.iter() {
	commands
	.entity(entity)
	.insert(materials.highlight_material.clone());
	}
	for entity in selected.iter() {
		commands
		.entity(entity)
		.insert(materials.select_material.clone());
	}
	for entity in selcted_highlighted.iter() {
		commands
			.entity(entity)
			.insert(materials.select_primary_material.clone());
	}
}
