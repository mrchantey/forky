use bevy::prelude::*;
use bevy_rapier3d::{
	prelude::*,
	rapier::prelude::{ImpulseJointSet, JointAxis, MotorModel, Vector},
};
use forky_core::{math::*, *};
use forky_play::{maze::*, *};
use sweet::*;

sweet! {
	it "works" {

		app::init()
		.insert_resource(MazeJointParams{
			target_pos:TAU * 0.125 * 0.5,
			stiffness:500.,
			damping:100.
		})
		.add_startup_system(spawn)
		.add_system(board_joint_controller)
		.run();
	}
}

fn spawn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let obj = commands
		.spawn_bundle(PbrBundle {
			transform: Transform::from_xyz(0., -0.1, 0.)
				.with_scale_xyz(10., 0.2, 10.),
			mesh: meshes.add(Mesh::from(shape::Cube::default())),
			..default()
		})
		.insert(Collider::cuboid(0.5, 0.5, 0.5))
		.insert(ColliderMassProperties::Density(0.))
		.id();


	let hinge_z =
		board_joint::spawn(&mut commands, &mut meshes, &mut materials);
	commands.entity(hinge_z).add_child(obj);
	// obj
}
