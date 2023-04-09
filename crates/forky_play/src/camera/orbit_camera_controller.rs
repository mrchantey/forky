//https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html
use bevy::input::mouse::*;
use bevy::prelude::*;
// use bevy::{input::mouse::*, render::camera::*};
// use bevy_easings;
use forky_core::math::*;

#[derive(Component)]
pub struct CameraParent;

#[derive(Component, Clone, Copy)]
pub struct OrbitController {
	/// The "focus point" to orbit around. It is automatically updated when panning the camera
	pub focus: Vec3,
	pub radius: f32,
	pub upside_down: bool,
}

// impl bevy_easings::Lerp for OrbitController {
// 	type Scalar = f32;
// 	fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
// 		OrbitController {
// 			radius: f32::lerp(&self.radius, &other.radius, scalar),
// 			..self.clone()
// 		}
// 	}
// }
impl Default for OrbitController {
	fn default() -> Self {
		OrbitController {
			focus: Vec3::ZERO,
			radius: 5.0,
			upside_down: false,
		}
	}
}

pub fn orbit_camera_controller(
	window: Query<&Window>,
	mouse: Res<Input<MouseButton>>,
	mut ev_motion: EventReader<MouseMotion>,
	mut ev_scroll: EventReader<MouseWheel>,
	mut query: Query<(&mut Transform, &mut OrbitController, &Projection)>,
) {
	let window = window.single();
	// change input mapping for orbit and panning here
	let orbit_button = MouseButton::Right;
	let pan_button = MouseButton::Middle;


	let mut pan = Vec2::ZERO;
	let mut rotation_move = Vec2::ZERO;
	let mut scroll = 0.0;
	let mut orbit_button_changed = false;

	if mouse.pressed(orbit_button) {
		for ev in ev_motion.iter() {
			rotation_move += ev.delta;
		}
	} else if mouse.pressed(pan_button) {
		// Pan only if we're not rotating at the moment
		for ev in ev_motion.iter() {
			pan += ev.delta;
		}
	}
	for ev in ev_scroll.iter() {
		scroll += ev.y;
	}
	if mouse.just_released(orbit_button) || mouse.just_pressed(orbit_button) {
		orbit_button_changed = true;
	}

	let window_size = Vec2::new(window.width(), window.height());
	for (mut tran, mut controller, projection) in query.iter_mut() {
		if orbit_button_changed {
			let up = tran.rotation * Vec3::Y;
			controller.upside_down = up.y <= 0.0;
		}

		let mut any = false;
		if rotation_move.length_squared() > 0.0 {
			any = true;
			let delta_x = {
				let delta = rotation_move.x / window_size.x * PI * 2.0;
				if controller.upside_down {
					-delta
				} else {
					delta
				}
			};
			let delta_y = rotation_move.y / window_size.y * PI;
			let yaw = Quat::from_rotation_y(-delta_x);
			let pitch = Quat::from_rotation_x(-delta_y);
			tran.rotation = yaw * tran.rotation; // rotate around global y axis
			tran.rotation = tran.rotation * pitch; // rotate around local x axis
		} else if pan.length_squared() > 0.0 {
			any = true;
			// make panning distance independent of resolution and FOV,
			if let Projection::Perspective(projection) = projection {
				pan *= Vec2::new(
					projection.fov * projection.aspect_ratio,
					projection.fov,
				) / window_size;
			}
			// translate by local axes
			let right = tran.rotation * Vec3::X * -pan.x;
			let up = tran.rotation * Vec3::Y * pan.y;
			// make panning proportional to distance away from focus point
			let translation = (right + up) * controller.radius;
			controller.focus += translation;
		} else if scroll.abs() > 0.0 {
			any = true;
			controller.radius -= scroll * controller.radius * 0.2;
			// dont allow zoom to reach zero or you get stuck
			controller.radius = f32::max(controller.radius, 0.05);
		}

		if any {
			update_translation_from_orbit(&mut tran, &controller);
		}
	}
}

// fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
// 	let window = windows.get_primary().unwrap();
// 	let window = Vec2::new(window.width() as f32, window.height() as f32);
// 	window
// }

pub fn update_translation_from_orbit(
	tran: &mut Mut<Transform>,
	orbit: &OrbitController,
) {
	let rot_matrix = Mat3::from_quat(tran.rotation);
	//invert orbit.radius if using camera parent
	tran.translation =
		orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbit.radius));
}

pub fn update_orbit_from_transform(
	orbit: &mut Mut<OrbitController>,
	tran: &Transform,
) {
	let radius = orbit.radius;
	orbit.focus = tran.forward() * radius;
}
