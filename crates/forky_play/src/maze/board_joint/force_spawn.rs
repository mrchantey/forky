use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::JointAxis};

pub fn force_spawn(commands: &mut Commands) -> Entity {
	let root = commands
		.spawn_bundle(SpatialBundle::from_xyz(0., 0., 0.))
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

	commands
		.spawn_bundle(SpatialBundle::from_xyz(0., 0., 0.))
		//rigidbody
		.insert(RigidBody::Dynamic)
		.insert(ImpulseJoint::new(*parent, joint))
		.insert(ExternalForce::default())
		.insert(Velocity::default())
		.insert(Damping {
			linear_damping: 0.,
			angular_damping: 0.9,
		})
		.insert(AdditionalMassProperties::one())
		//constraints
		.insert(GravityScale(0.))
		.insert(LockedAxes::TRANSLATION_LOCKED)
		.insert(Dominance::group(1))
		//misc
		.insert(MazeJoint { axis })
		.id()
}
