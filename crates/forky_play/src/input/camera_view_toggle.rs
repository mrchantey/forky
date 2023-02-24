use bevy::prelude::*;
//https://bevy-cheatbook.github.io/input/keyboard.html

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
// #[derive(Eq)]
#[derive(Resource, Component)]
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
	keys: Res<Input<KeyCode>>,
	query: Query<(&mut Camera, &CameraViewType)>,
) {
	if keys.just_pressed(KeyCode::Tab) {
		let next_state = next_toggle_state(&cam_type);
		cam_type.clone_from(&next_state);
		run_camera_view_toggle(cam_type, query);
	}
}
pub fn run_camera_view_toggle(
	cam_type: ResMut<CameraViewType>,
	mut query: Query<(&mut Camera, &CameraViewType)>,
) {
	for (mut cam, state) in query.iter_mut() {
		if *state == *cam_type {
			cam.is_active = true
		} else {
			cam.is_active = false
		}
	}
}
