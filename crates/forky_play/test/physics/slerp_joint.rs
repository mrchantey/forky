use bevy::prelude::*;
use forky_play::{utility::surrender_focus, *};
use sweet::*;

sweet! {
	it skip "works" {

		app::init()
		.add_plugin(physics::SlerpJointPlugin)
		.add_startup_system(surrender_focus)
		.add_startup_system(my_startup_system)
		.run();
	}
}

fn my_startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
	commands
		.spawn_bundle(PbrBundle {
			transform: Transform::from_xyz(0., -0.1, 0.),
			mesh: meshes.add(Mesh::from(shape::Cube::default())),
			..default()
		})
		.insert(physics::SlerpTarget::default());
}
