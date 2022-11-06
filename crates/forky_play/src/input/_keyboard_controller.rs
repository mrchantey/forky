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
		let t_delta = kb.translate_speed * time.delta_seconds();
		let r_delta = (kb.rotate_speed * time.delta_seconds()) / TAU;
		let t = parse_keyboard_translation(&keys) * t_delta;
		let r = parse_keyboard_rotation(&keys) * r_delta;
		tran.translate_flat_x(t.x);
		tran.translate_flat_y(t.y);
		tran.translate_flat_z(t.z);
		tran.rotate_y(r.y);
	}
}

pub fn parse_keyboard_translation(keys: &Res<Input<KeyCode>>) -> Vec3 {
	let mut t = Vec3::default();
	if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
		t.z -= 1.;
	}
	if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
		t.z += 1.;
	}
	if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
		t.x -= 1.;
	}
	if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
		t.x += 1.;
	}
	if keys.pressed(KeyCode::R) {
		t.y += 1.;
	}
	if keys.pressed(KeyCode::F) {
		t.y -= 1.;
	}
	t
}
pub fn parse_keyboard_rotation(keys: &Res<Input<KeyCode>>) -> Vec3 {
	let mut r = Vec3::default();
	if keys.pressed(KeyCode::Q) {
		r.y += 1.;
	}
	if keys.pressed(KeyCode::E) {
		r.y -= 1.;
	}
	r
}
