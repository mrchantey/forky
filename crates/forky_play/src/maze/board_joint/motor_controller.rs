use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::JointAxis;

pub fn motor_controller(
	params: Res<MazeJointParams>,
	keys: Res<ButtonInput<KeyCode>>,
	mut query: Query<(&mut ImpulseJoint, &MazeJoint)>,
) {
	for (mut joint, maze_joint) in query.iter_mut() {
		let mut target = 0.;
		match maze_joint.axis {
			JointAxis::X => {
				if keys.pressed(KeyCode::KeyI) {
					target += params.max_angle;
				}
				if keys.pressed(KeyCode::KeyK) {
					target -= params.max_angle;
				}
			}
			JointAxis::Z => {
				if keys.pressed(KeyCode::KeyJ) {
					target += params.max_angle;
				}
				if keys.pressed(KeyCode::KeyL) {
					target -= params.max_angle;
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
