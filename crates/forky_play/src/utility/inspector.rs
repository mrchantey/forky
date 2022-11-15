use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorParams;


pub fn toggle_inspector_on_keypress(
	mut params: ResMut<WorldInspectorParams>,
	keys: Res<Input<KeyCode>>,
) {
	if keys.any_just_pressed([KeyCode::G]) {
		params.enabled = !params.enabled;
	}
}
