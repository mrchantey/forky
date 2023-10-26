use bevy::prelude::*;
use forky_core::math::*;
//https://bevy-cheatbook.github.io/input/keyboard.html
use super::*;
use crate::*;

pub fn orbit_keyboard_controller(
	time: Res<Time>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<(
		&input::TransformController,
		&mut OrbitController,
		&mut Transform,
	),With<ActiveCamera>>,
) {
	for (kb, mut orbit, mut tran) in query.iter_mut() {
		let pos_scalar = kb.translate_speed * time.delta_seconds();
		let pos_delta = input::parse_keyboard_translation(&keys) * pos_scalar;
		orbit.focus += tran.flat_x() * pos_delta.x;
		orbit.focus += tran.flat_y() * pos_delta.y;
		orbit.focus += tran.flat_z() * pos_delta.z;
		update_translation_from_orbit(&mut tran, &orbit);

		let rot_scalar = (kb.rotate_speed * time.delta_seconds()) / TAU;
		let rot_delta = input::parse_keyboard_rotation(&keys) * rot_scalar;
		tran.rotate_around(orbit.focus, Quat::from_rotation_y(rot_delta.y));
		// tran.rotation = Quat::from_rotation_y(r.y) * tran.rotation;
		// tran.rotate_y(r.y);
	}
}
