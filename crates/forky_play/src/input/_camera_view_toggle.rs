use bevy::{prelude::*, utils::HashMap};
use forky_core::{math::*, *};
//https://bevy-cheatbook.github.io/input/keyboard.html
use super::*;
use crate::*;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
// #[derive(Eq)]
#[derive(Component)]
pub enum CameraViewType {
	Orbit,
	Top,
	Front,
	Right,
	// FPS,
}

fn next_toggle_state(prev: &CameraViewType) -> CameraViewType {
	match prev {
		CameraViewType::Orbit => CameraViewType::Top,
		CameraViewType::Top => CameraViewType::Front,
		CameraViewType::Front => CameraViewType::Right,
		CameraViewType::Right => CameraViewType::Orbit,
		// CameraViewType::FPS => CameraViewType::Orbit,
	}
}


pub fn camera_view_toggle(
	mut cam_type: ResMut<CameraViewType>,
	time: Res<Time>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<(&mut Camera, &CameraViewType)>,
) {
	if !keys.just_pressed(KeyCode::Tab) {
		return;
	}

	let next_state = next_toggle_state(&cam_type);
	cam_type.clone_from(&next_state);

	for (mut cam, state) in query.iter_mut() {
		if *state == *cam_type {
			cam.is_active = true
		} else {
			cam.is_active = false
		}
	}
}
