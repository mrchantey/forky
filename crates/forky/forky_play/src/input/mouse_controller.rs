use super::*;
use crate::*;
use bevy::input::mouse::*;
use bevy::prelude::*;


#[cfg(not(target_arch = "wasm32"))]
pub const SCROLL_SPEED: f32 = 0.1;
#[cfg(target_arch = "wasm32")]
pub const SCROLL_SPEED: f32 = 0.00001;

pub fn mouse_controller(
	mut ev_motion: EventReader<MouseMotion>,
	mut ev_scroll: EventReader<MouseWheel>,
	input_mouse: Res<Input<MouseButton>>,
	mut query: Query<
		(&TransformController, &mut Transform),
		With<ActiveTransformController>,
	>,
) {
	for (controller, mut tran) in query.iter_mut() {
		if controller.allow_rotation && input_mouse.pressed(MouseButton::Left) {
			for ev in ev_motion.read() {
				//TODO local axis
				tran.rotate_y(ev.delta.x * controller.rotate_speed * 0.0001);
				tran.rotate_local_x(
					-ev.delta.y * controller.rotate_speed * 0.0001,
				);
			}
		}

		for ev in ev_scroll.read() {
			let scalar = ev.y * controller.translate_speed * SCROLL_SPEED;
			if controller.local_axis {
				let axis = tran.forward();
				tran.translate_local(axis * scalar);
			} else {
				tran.translate_flat_y(scalar);
			}
		}
	}
}
