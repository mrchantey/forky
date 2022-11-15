use bevy::prelude::*;
use bevy_rapier3d::{
	prelude::*,
	rapier::prelude::{MotorModel, Vector},
};
use forky_core::{math::*, *};
use forky_play::{utility::surrender_focus, *};
use sweet::*;

sweet! {
	it "works" {

		app::init()
		.add_startup_system(surrender_focus)
		.add_startup_system(my_startup_system)
		.add_system(my_system)
		.run();
	}
}

fn my_startup_system(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let parent = commands
		.spawn_bundle(SpatialBundle::default())
		.insert(Collider::cuboid(0.5, 0.5, 0.5))
		.insert(RigidBody::Fixed)
		.insert(GravityScale(0.))
		.id();


	let joint = RevoluteJointBuilder::new(Vec3::Z)
		.local_anchor1(Vec3::new(0.0, 0.0, 0.0))
		.local_anchor2(Vec3::new(0.0, -2.0, 0.0))
		.motor_position(TAU * 0.125,1000.,100.)
		// .motor_velocity(10., 10000.)//0.0001 = never, 10000||0 = now
		;

	commands
		.spawn_bundle(SpatialBundle {
			transform: Transform::from_xyz(0., 2., 0.),
			..default()
		})
		.insert(Collider::cuboid(0.5, 0.5, 0.5))
		.insert(GravityScale(0.))
		.insert(RigidBody::Dynamic)
		.insert(ImpulseJoint::new(parent, joint));
}


fn my_system(
	time: Res<Time>,
	mut query: Query<(&mut ImpulseJoint, &Transform)>,
) {
	for (mut joint, tran) in query.iter_mut() {
		// joint.data.mot
		// joint.mo
	}
}
