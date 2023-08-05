use crate::tool::*;
use bevy::prelude::*;


#[rustfmt::skip]
pub fn select_entities(
	mut commands: Commands,
	keys: Res<Input<KeyCode>>,
	camera_ray:Res<CameraRay>,
	mouse: Res<Input<MouseButton>>,
	selected: Query<Entity, With<Selected>>,
) {
	let multi_select = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

/*
if ray hit
	if press
		select the hit
		if no multi select, remove all others
else if release & no multi select
		remove all selected
*/

	if let Some(primary) = camera_ray.entity{
		if mouse.just_pressed(SELECT_BUTTON) {
			commands.entity(primary).insert(Selected);
			if !multi_select {
				for entity in selected.iter().filter(|e| *e != primary) {
						commands.entity(entity).remove::<Selected>();
				}
			}
		}
	}else	if mouse.just_pressed(SELECT_BUTTON) && !multi_select{
		for entity in selected.iter(){
			commands.entity(entity).remove::<Selected>();
		}
	}
}
