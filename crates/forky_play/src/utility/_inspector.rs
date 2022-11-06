use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorParams;
use forky_core::{math::*, *};


pub fn toggle_inspector_on_keypress(
	mut params: ResMut<WorldInspectorParams>,
	keys: Res<Input<KeyCode>>,
) {
	if keys.any_just_pressed([KeyCode::G]) {
		params.enabled = !params.enabled;
	}
}
