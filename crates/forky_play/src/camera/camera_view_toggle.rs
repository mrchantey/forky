use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::input::ActiveTransformController;
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
	mut commands: Commands,
	cam_type: Option<ResMut<CameraViewType>>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<(Entity, &mut Camera, &CameraViewType)>,
) {
	let cam_type = match cam_type {
		Some(value) => value,
		None => return,
	};

	if keys.just_pressed(KeyCode::Tab) {
		let next_state = next_toggle_state(&cam_type);
		commands.insert_resource(next_state);
		// cam_type.clone_from(&next_state);
		run_camera_view_toggle(&mut commands, &cam_type, &mut query);
	}
}
pub fn toggle_startup_camera(
	mut commands: Commands,
	cam_type: Option<ResMut<CameraViewType>>,
	mut query: Query<(Entity, &mut Camera, &CameraViewType)>,
) {
	if let Some(cam_type) = cam_type {
		run_camera_view_toggle(&mut commands, &cam_type, &mut query);
		return;
	}
	match query.get_single() {
		Ok((_, _, state)) => {
			run_camera_view_toggle(&mut commands, &state.clone(), &mut query);
		}
		Err(QuerySingleError::NoEntities(_)) => {}
		Err(QuerySingleError::MultipleEntities(_)) => {
			panic!("multiple cameras found but no CameraViewType resource set");
		}
	}
}

pub fn run_camera_view_toggle(
	commands: &mut Commands,
	cam_type: &CameraViewType,
	query: &mut Query<(Entity, &mut Camera, &CameraViewType)>,
) {
	for (entity, mut cam, state) in query.iter_mut() {
		if state == cam_type {
			cam.is_active = true;
			commands.entity(entity).insert(ActiveCamera);
			if state != &CameraViewType::Orbit {
				commands.entity(entity).insert(ActiveTransformController);
			}
		} else {
			commands
				.entity(entity)
				.remove::<(ActiveCamera, ActiveTransformController)>();
			cam.is_active = false
		}
	}
}
