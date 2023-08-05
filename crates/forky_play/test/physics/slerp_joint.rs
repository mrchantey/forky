use bevy::prelude::*;
use forky_play::utility::surrender_focus;
use forky_play::*;
use sweet::*;

sweet! {
	it skip "works" {

		App::new()
			.add_plugins(plugins::ForkyFullPlugin::default())
			.add_plugins(physics::SlerpJointPlugin)
			.add_systems(Startup, surrender_focus)
			.add_systems(Startup, my_startup_system)
			.run();
	}
}

fn my_startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
	commands
		.spawn(PbrBundle {
			transform: Transform::from_xyz(0., -0.1, 0.),
			mesh: meshes.add(Mesh::from(shape::Cube::default())),
			..default()
		})
		.insert(physics::SlerpTarget::default());
}
