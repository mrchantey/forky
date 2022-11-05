use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math::*, *};


#[derive(Component)]
pub struct Steer1 {
	position: f32,
	target: f32,
	velocity: f32,
	max_velocity: f32,
	steering: f32,
}




fn update_steering(mut query: Query<&mut Steer1>) {
	for mut steer in query.iter_mut() {
		let delta = steer.target - steer.position;
		let normal = delta.signum();

		let desired_velocity = normal * steer.max_velocity;
		steer.steering = desired_velocity - steer.velocity;
	}
}
