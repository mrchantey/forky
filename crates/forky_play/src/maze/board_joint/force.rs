use crate::{
	maze::{board::MazeBoardTag, mesh_shape},
	*,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math::*, *};


pub fn spawn(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
	commands
		.spawn_bundle(SpatialBundle::default())
		// .push_children(&[walls, floor])
		.insert(MazeBoardTag)
		.insert(RigidBody::Dynamic)
		.insert(Damping {
			linear_damping: 0.,
			angular_damping: 0.8,
		})
		.insert(LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED_Y)
		.insert(GravityScale(0.))
		.insert(ExternalForce::default())
		.id()
}


pub fn force_controller(
	keys: Res<Input<KeyCode>>,
	mut query: Query<(&mut ExternalForce, &Transform), With<MazeBoardTag>>,
) {
	let mut torque = Vec3::ZERO;
	let force = 10.;
	if keys.pressed(KeyCode::I) {
		torque.x += force;
	}
	if keys.pressed(KeyCode::K) {
		torque.x -= force;
	}
	if keys.pressed(KeyCode::J) {
		torque.z -= force;
	}
	if keys.pressed(KeyCode::L) {
		torque.z += force;
	}

	for (mut force, tran) in query.iter_mut() {
		// force.torque += Vec3::new(1000.,0.,0.);
		force.torque = torque;
	}
}
