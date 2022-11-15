use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxis};

pub fn motor_spawn(commands: &mut Commands) -> Entity {
	let root = commands
		.spawn_bundle(SpatialBundle {
			transform: Transform::from_xyz(0., 0., 0.),
			..default()
		})
		.insert(RigidBody::Fixed)
		.id();


	let hinge_x = spawn_revolute_joint(commands, &root, JointAxis::X);
	let hinge_z = spawn_revolute_joint(commands, &hinge_x, JointAxis::Z);

	hinge_z
}


fn spawn_revolute_joint(
	commands: &mut Commands,
	parent: &Entity,
	axis: JointAxis,
) -> Entity {
	let vec = match axis {
		JointAxis::X => Vec3::X,
		JointAxis::Y => Vec3::Y,
		JointAxis::Z => Vec3::Z,
		JointAxis::AngX => Vec3::X,
		JointAxis::AngY => Vec3::Y,
		JointAxis::AngZ => Vec3::Z,
	};

	let joint = RevoluteJointBuilder::new(vec);
	// .local_anchor2(Vec3::new(0.0, -0.1, 0.0));
	let ijoint = ImpulseJoint::new(*parent, joint);

	// ijoint.data = *ijoint.data.set_motor_velocity(axis, 10., 1.);
	// ijoint
	// 	.data
	// 	.as_revolute_mut()
	// 	.unwrap()
	// 	.set_motor_velocity(10., 1.);
	// .set_motor_position(TAU * 0.125 * 0.5, 500., 100.);
	// ijoint.data

	commands
		.spawn_bundle(SpatialBundle::from_xyz(0., 0., 0.))
		.insert(MazeJoint { axis })
		.insert(Collider::ball(0.))
		.insert(ColliderMassProperties::Density(1.0))
		.insert(GravityScale(0.))
		.insert(AdditionalMassProperties::MassProperties(MassProperties {
			principal_inertia: Vec3::ONE,
			mass: 1.,
			..default()
		}))
		// .insert(AdditionalMassProperties::Mass(1.))
		.insert(LockedAxes::TRANSLATION_LOCKED)
		.insert(RigidBody::Dynamic)
		.insert(Dominance::group(1))
		.insert(ijoint)
		.id()
}
