use bevy::prelude::*;
use bevy_rapier3d::{rapier::prelude::JointAxis};
use forky_core::{math::*};


pub struct MazeJointParams {
	pub max_angle: f32,
	pub stiffness: f32,
	pub damping: f32,
}

#[derive(Component)]
pub struct MazeJoint {
	pub axis: JointAxis,
}


impl Default for MazeJointParams {
	fn default() -> Self {
		Self {
			max_angle: QUARTER_TAU * 0.1,
			stiffness: 100.,
			damping: 0.2,
		}
	}
}
