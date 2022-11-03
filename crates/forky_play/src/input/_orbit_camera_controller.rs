use bevy::{input::mouse::*, prelude::*, render::camera::*};
//https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

use crate::*;
use bevy::prelude::*;
use forky_core::math::*;

#[derive(Component)]
pub struct CameraParent;

#[derive(Component,Clone, Copy)]
pub struct OrbitController {
	/// The "focus point" to orbit around. It is automatically updated when panning the camera
	pub focus: Vec3,
	pub radius: f32,
	pub upside_down: bool,
}

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
	windows: Res<Windows>,
	mouse: Res<Input<MouseButton>>,
	mut ev_motion: EventReader<MouseMotion>,
	mut ev_scroll: EventReader<MouseWheel>,
	mut q_cam: Query<(&Parent, &mut Transform, &Projection)>,
	mut q_parent: Query<(&mut OrbitController, &mut Transform), Without<Projection>>,
) {
	// change input mapping for orbit and panning here
	let orbit_button = MouseButton::Left;
	let pan_button = MouseButton::Middle;

	let mut pan = Vec2::ZERO;
	let mut rotation_move = Vec2::ZERO;
	let mut scroll = 0.0;
	let mut orbit_button_changed = false;
	let invert_y = -1.;

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

	for (mut e_parent, mut tran_child, projection) in q_cam.iter_mut() {
		let (mut pan_orbit, mut tran_parent) = q_parent.get_mut(e_parent.get()).unwrap();

		if orbit_button_changed {
			let up = tran_parent.rotation * Vec3::Y;
			pan_orbit.upside_down = up.y <= 0.0;
		}

		let mut any = false;
		if rotation_move.length_squared() > 0.0 {
			any = true;
			let window = get_primary_window_size(&windows);
			let delta_x = {
				let delta = rotation_move.x / window.x * PI * 2.0;
				if pan_orbit.upside_down {
					-delta
				} else {
					delta
				}
			};
			let delta_y = rotation_move.y / window.y * PI * invert_y;
			let yaw = Quat::from_rotation_y(-delta_x);
			let pitch = Quat::from_rotation_x(-delta_y);
			tran_parent.rotation = yaw * tran_parent.rotation; // rotate around global y axis
			tran_parent.rotation = tran_parent.rotation * pitch; // rotate around local x axis
		} else if pan.length_squared() > 0.0 {
			any = true;
			// make panning distance independent of resolution and FOV,
			let window = get_primary_window_size(&windows);
			if let Projection::Perspective(projection) = projection {
				pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
			}
			// translate by local axes
			let right = tran_parent.rotation * Vec3::X * -pan.x;
			let up = tran_parent.rotation * Vec3::Y * pan.y;
			// make panning proportional to distance away from focus point
			let translation = (right + up) * pan_orbit.radius;
			pan_orbit.focus += translation;
		} else if scroll.abs() > 0.0 {
			any = true;
			pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
			// dont allow zoom to reach zero or you get stuck
			pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
		}

		if any {
			update_translation_from_orbit(&mut tran_parent, &pan_orbit);
		}
	}
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
	let window = windows.get_primary().unwrap();
	let window = Vec2::new(window.width() as f32, window.height() as f32);
	window
}

pub fn update_translation_from_orbit(tran:&mut Mut<Transform>,orbit:&OrbitController){
	let rot_matrix = Mat3::from_quat(tran.rotation);
	//inverted orbit.radius because camera parent
	tran.translation =
		orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, -orbit.radius));
}

pub fn update_orbit_from_transform(orbit:&mut Mut<OrbitController>, tran:&Transform){
	let r = orbit.radius;
	orbit.focus = tran.local_z() * r;
}