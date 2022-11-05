use std::ops::Add;

use bevy::{prelude::*, transform};


#[derive(Component)]
pub struct KinematicBody {
	pub velocity: Vec3,
	pub acceleration: Vec3,
	pub angular_velocity: Vec3,
	pub angular_acceleration: Vec3,
}

fn add(){
	// body.velocity += body.acceleration;
}

pub fn update_kinematic_bodies(
	time: Res<Time>,
	mut query: Query<(&mut Transform, &mut KinematicBody)>,
) {
	let delta = time.delta_seconds();
	for (mut transform, mut body) in query.iter_mut() {
		let a = body.acceleration;
		body.velocity += a;
		transform.translation += body.velocity * delta;
	}
}
