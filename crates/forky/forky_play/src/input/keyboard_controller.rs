//https://bevy-cheatbook.github.io/input/keyboard.html
use super::*;
use crate::*;
use bevy::prelude::*;
use forky_core::math::TAU;

pub fn keyboard_controller(
	time: Res<Time<Real>>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<
		(&TransformController, &mut Transform),
		With<ActiveTransformController>,
	>,
) {
	for (controller, mut tran) in query.iter_mut() {
		let pos_scalar = controller.translate_speed * time.delta_seconds();
		let mut pos_delta = parse_keyboard_translation(&keys) * pos_scalar;
		let rot_scalar = (controller.rotate_speed * time.delta_seconds()) / TAU;
		let rot_delta = parse_keyboard_rotation(&keys) * rot_scalar;
		if controller.local_axis {
			pos_delta.swap_yz().negate_y().negate_z();
			tran.translate_local(pos_delta);
			if controller.allow_rotation {
				let axis = tran.up();
				tran.rotate_axis(axis, rot_delta.y);
			}
		} else {
			tran.translate_flat_x(pos_delta.x);
			tran.translate_flat_y(pos_delta.y);
			tran.translate_flat_z(pos_delta.z);
			if controller.allow_rotation {
				tran.rotate_y(rot_delta.y);
			}
		}
	}
}

pub fn parse_keyboard_translation(keys: &Res<Input<KeyCode>>) -> Vec3 {
	let mut pos_delta = Vec3::default();
	if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
		pos_delta.z -= 1.;
	}
	if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
		pos_delta.z += 1.;
	}
	if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
		pos_delta.x -= 1.;
	}
	if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
		pos_delta.x += 1.;
	}
	if keys.pressed(KeyCode::R) {
		pos_delta.y += 1.;
	}
	if keys.pressed(KeyCode::F) {
		pos_delta.y -= 1.;
	}
	pos_delta
}
pub fn parse_keyboard_rotation(keys: &Res<Input<KeyCode>>) -> Vec3 {
	let mut rot_delta = Vec3::default();
	if keys.pressed(KeyCode::Q) {
		rot_delta.y += 1.;
	}
	if keys.pressed(KeyCode::E) {
		rot_delta.y -= 1.;
	}
	rot_delta
}
