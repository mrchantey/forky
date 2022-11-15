use bevy::prelude::*;


#[derive(Component)]
pub struct Steer1 {
	pub position: f32,
	pub target: f32,
	pub velocity: f32,
	pub max_velocity: f32,
	pub steering: f32,
}




pub fn update_steering(mut query: Query<&mut Steer1>) {
	for mut steer in query.iter_mut() {
		let delta = steer.target - steer.position;
		let normal = delta.signum();

		let desired_velocity = normal * steer.max_velocity;
		steer.steering = desired_velocity - steer.velocity;
	}
}
