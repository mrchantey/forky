use super::*;
use crate::*;
use bevy::{input::mouse::*, prelude::*};

pub fn mouse_controller(
	mut ev_motion: EventReader<MouseMotion>,
	mut ev_scroll: EventReader<MouseWheel>,
	input_mouse: Res<Input<MouseButton>>,
	mut query: Query<(&TransformController, &mut Transform)>,
) {
	// let t = time.de
	for (param, mut tran) in query.iter_mut() {
		if input_mouse.pressed(MouseButton::Left) {
			for ev in ev_motion.iter() {
				tran.rotate_y(ev.delta.x * param.rotate_speed * 0.0001);
				tran.rotate_local_x(-ev.delta.y * param.rotate_speed * 0.0001);
			}
		}

		for ev in ev_scroll.iter() {
			tran.translate_flat_y(ev.y * param.translate_speed * 0.1);
		}
	}
}
