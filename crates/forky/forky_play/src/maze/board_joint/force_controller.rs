use super::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::JointAxis;


// fn max_

pub fn force_controller(
	params: Res<MazeJointParams>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<(&mut ExternalForce, &Velocity, &Transform, &MazeJoint)>,
) {
	for (mut force, velocity, tran, joint) in query.iter_mut() {
		let mut torque = Vec3::ZERO;
		let (x, _y, z) = tran.rotation.to_euler(EulerRot::XYZ);
		match joint.axis {
			JointAxis::X => {
				torque.x = solve_torque(
					x,
					velocity.angvel.x,
					keys.pressed(KeyCode::K),
					keys.pressed(KeyCode::I),
					&params,
				)
			}
			JointAxis::Z => {
				torque.z = solve_torque(
					z,
					velocity.angvel.z,
					keys.pressed(KeyCode::J),
					keys.pressed(KeyCode::L),
					&params,
				)
			}
			_ => {}
		}
		force.torque = torque;
	}
}

fn solve_torque(
	position: f32,
	velocity: f32,
	left_down: bool,
	right_down: bool,
	params: &MazeJointParams,
) -> f32 {
	let mut desired = 0.;
	if left_down {
		desired += params.max_angle - position;
	}
	if right_down {
		desired += -params.max_angle - position;
	}
	if !left_down && !right_down {
		desired += -position;
	}
	return (desired - velocity * params.damping) * params.stiffness;
}
