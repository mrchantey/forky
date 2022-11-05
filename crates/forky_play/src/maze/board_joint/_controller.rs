use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxis};
use forky_core::{math::*, *};


pub struct MazeJointParams {
	pub target_pos: f32,
	pub stiffness: f32,
	pub damping: f32,
}
#[derive(Component)]
pub struct MazeJoint {
	pub axis: JointAxis,
}


pub fn controller(
	params: Res<MazeJointParams>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<(&mut ImpulseJoint, &MazeJoint)>,
) {
	let mut torque = Vec3::ZERO;
	let force = 10.;

	for (mut joint, MazeJoint { axis }) in query.iter_mut() {
		let mut target = 0.;
		match axis {
			JointAxis::X => {
				if keys.pressed(KeyCode::I) {
					target += params.target_pos;
				}
				if keys.pressed(KeyCode::K) {
					target -= params.target_pos;
				}
			}
			JointAxis::Z => {
				if keys.pressed(KeyCode::J) {
					target += params.target_pos;
				}
				if keys.pressed(KeyCode::L) {
					target -= params.target_pos;
				}
			}
			_ => {}
		}
		joint.data.as_revolute_mut().unwrap().set_motor_position(
			target,
			params.stiffness,
			params.damping,
		);
	}
}
