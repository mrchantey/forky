use bevy::prelude::*;
use forky_core::math::TAU;
//https://bevy-cheatbook.github.io/input/keyboard.html
use super::*;
use crate::*;

pub fn keyboard_controller(
	time: Res<Time>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<(&TransformController, &mut Transform)>,
) {
	for (kb, mut tran) in query.iter_mut() {
		let pos_scalar = kb.translate_speed * time.delta_seconds();
		let pos_delta = parse_keyboard_translation(&keys) * pos_scalar;
		tran.translate_flat_x(pos_delta.x);
		tran.translate_flat_y(pos_delta.y);
		tran.translate_flat_z(pos_delta.z);
		let rot_scalar = (kb.rotate_speed * time.delta_seconds()) / TAU;
		let rot_delta = parse_keyboard_rotation(&keys) * rot_scalar;
		tran.rotate_y(rot_delta.y);
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
