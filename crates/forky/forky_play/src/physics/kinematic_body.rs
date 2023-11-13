use bevy::prelude::*;


#[derive(Component)]
pub struct KinematicBody {
	pub position: Vec3,
	pub rotation: Vec3,
	pub velocity: Vec3,
	pub acceleration: Vec3,
	pub angular_velocity: Vec3,
	pub angular_acceleration: Vec3,
}

// fn add() {
// 	// body.velocity += body.acceleration;
// }
pub fn update_kinematic_bodies(
	time: Res<Time<Real>>,
	mut query: Query<(&mut Transform, &mut KinematicBody)>,
) {
	let delta = time.delta_seconds();
	for (mut transform, mut body) in query.iter_mut() {
		let a = body.acceleration;
		body.velocity += a;
		transform.translation += body.velocity * delta;
	}
}
