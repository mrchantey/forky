use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
// use forky_core::{math::*, *};
use forky_play::{
	maze::*,
	*,
};

fn main() {
	App::new()
		.add_plugins(plugins::ForkyFullPlugin::default())
		.insert_resource(board_joint::MazeJointParams::default())
		.add_systems(Startup, spawn)
		.add_systems(Update, board_joint::force_controller)
		.run();
}

fn spawn(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
	let obj = commands
		.spawn(PbrBundle {
			transform: Transform::from_xyz(0., -0.1, 0.)
				.with_scale_xyz(10., 0.2, 10.),
			mesh: meshes.add(Mesh::from(Cuboid::default())),
			..default()
		})
		.insert(Collider::cuboid(0.5, 0.5, 0.5))
		.insert(ColliderMassProperties::Density(0.))
		.id();


	let hinge_z = board_joint::force_spawn(&mut commands);
	commands.entity(hinge_z).add_child(obj);
	// obj
}
