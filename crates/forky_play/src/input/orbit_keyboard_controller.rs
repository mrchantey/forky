use bevy::prelude::*;
use forky_core::math::*;
//https://bevy-cheatbook.github.io/input/keyboard.html
use super::*;
use crate::*;

pub fn orbit_keyboard_controller(
	time: Res<Time>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<(
		&TransformController,
		&mut OrbitController,
		&mut Transform,
	)>,
) {
	for (kb, mut orbit, mut tran) in query.iter_mut() {
		let t_delta = kb.translate_speed * time.delta_seconds();
		let r_delta = (kb.rotate_speed * time.delta_seconds()) / TAU;
		let t = parse_keyboard_translation(&keys) * t_delta;
		let r = parse_keyboard_rotation(&keys) * r_delta;
		orbit.focus += tran.flat_x() * t.x;
		orbit.focus += tran.flat_y() * t.y;
		orbit.focus += tran.flat_z() * t.z;

		update_translation_from_orbit(&mut tran, &orbit);

		tran.rotate_around(orbit.focus, Quat::from_rotation_y(r.y));
		// tran.rotation = Quat::from_rotation_y(r.y) * tran.rotation;
		// tran.rotate_y(r.y);
	}
}
