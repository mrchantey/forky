use bevy::prelude::*;
//https://bevy-cheatbook.github.io/input/keyboard.html


#[derive(Component)]
pub struct ActiveCamera;


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
// #[derive(Eq)]
#[derive(Resource, Component, Default)]
pub enum CameraViewType {
	#[default]
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
	commands: Commands,
	mut cam_type: ResMut<CameraViewType>,
	keys: Res<Input<KeyCode>>,
	query: Query<(Entity, &mut Camera, &CameraViewType)>,
) {
	if keys.just_pressed(KeyCode::Tab) {
		let next_state = next_toggle_state(&cam_type);
		cam_type.clone_from(&next_state);
		run_camera_view_toggle(commands, cam_type, query);
	}
}
pub fn run_camera_view_toggle(
	mut commands: Commands,
	cam_type: ResMut<CameraViewType>,
	mut query: Query<(Entity, &mut Camera, &CameraViewType)>,
) {
	for (entity, mut cam, state) in query.iter_mut() {
		if *state == *cam_type {
			cam.is_active = true;
			commands.entity(entity).insert(ActiveCamera);
		} else {
			commands.entity(entity).remove::<ActiveCamera>();
			cam.is_active = false
		}
	}
}
