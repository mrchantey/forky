use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;

#[rustfmt::skip]
pub fn highlight_entities(
	mut commands: Commands,
	add_query: Query<Entity,(With<Hovered>, Without<Selected>, Without<Highlighted>)>,
	remove_query: Query<Entity,(With<Highlighted>, Without<Hovered>)>,
) {
	for entity in add_query.iter() {
		commands.entity(entity).insert(Highlighted);
	}
	for entity in remove_query.iter() {
		commands.entity(entity).remove::<Highlighted>();
	}
}
