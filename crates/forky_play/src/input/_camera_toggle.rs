use bevy::{prelude::*, utils::HashMap};
use forky_core::{math::*, *};
//https://bevy-cheatbook.github.io/input/keyboard.html
use super::*;
use crate::*;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
// #[derive(Eq)]
pub enum ToggleState {
	Orbit,
	Top,
	Front,
	Right,
	FPS,
}

fn next_toggle_state(prev: &ToggleState) -> ToggleState {
	match prev {
		ToggleState::Orbit => ToggleState::Top,
		ToggleState::Top => ToggleState::Front,
		ToggleState::Front => ToggleState::Right,
		ToggleState::Right => ToggleState::FPS,
		ToggleState::FPS => ToggleState::Orbit,
	}
}

pub struct OrbitCameraState {
	pub pose: Pose,
	pub controller: OrbitController,
}

impl Default for OrbitCameraState {
	fn default() -> Self {
		OrbitCameraState {
			pose: Pose::default(),
			controller: OrbitController::default(),
		}
	}
}

#[derive(Component)]
pub struct CameraToggle {
	pub state: ToggleState,
	pub poses: HashMap<ToggleState, OrbitCameraState>,
}

impl CameraToggle {}

const DIST: f32 = 15.;

impl Default for CameraToggle {
	fn default() -> Self {
		CameraToggle {
			state: ToggleState::Orbit,
			poses: HashMap::from([
				(
					ToggleState::Orbit,
					OrbitCameraState{pose: Pose {
						position: Vec3::new(0., 5., -10.),
						rotation: Quat::default(),
					},
					..default()

					},
				),
				(ToggleState::FPS, OrbitCameraState::default()),
				(
					ToggleState::Top,
					OrbitCameraState{ pose: Pose {
						position: Vec3::Y * DIST,
						rotation: Quat::from_down(),
					},
					..default()
					},
				),
				(
					ToggleState::Front,
					OrbitCameraState{pose: Pose {
						position: -Vec3::Z * DIST,
						rotation: Quat::from_forward(),
					},..default()
					},
				),
				(
					ToggleState::Right,
					OrbitCameraState{pose: Pose {
						position: -Vec3::X * DIST,
						rotation: Quat::from_right(),
					},..default()
					},
				),
			]),
		}
	}
}


pub fn camera_toggle(
	time: Res<Time>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<(&mut CameraToggle, &mut OrbitController, &mut Transform)>,
) {
	for (mut toggle, mut orbit, mut tran) in query.iter_mut() {
		if !keys.just_pressed(KeyCode::Tab) {
			continue;
		}
		let prev_key = toggle.state.clone();
		let mut prev_val = toggle.poses.get_mut(&prev_key).unwrap();
		prev_val.pose.set_from_transform(&tran);
		prev_val.controller.clone_from(&orbit);

		let next_key = next_toggle_state(&prev_key);
		let mut next_val = toggle.poses.get_mut(&next_key).unwrap();
		// log!(next_state == ToggleState::Top);
		// log!("here");
		dir!(next_key);
		dir!(next_val.pose);
		tran.from_pose(&next_val.pose);
		orbit.clone_from(&next_val.controller);
		toggle.state = next_key;
		// update_orbit_from_transform(&mut orbit, &tran);
	}
}
