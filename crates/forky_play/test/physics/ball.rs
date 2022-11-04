use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::*;
use forky_play::maze::{ball, maze_floor};
use forky_play::OptI32X;
use forky_play::*;
use sweet::*;

sweet! {
	it "works" {
		app::init()
		// .forky_exit_after(5)
		.insert_resource(Msaa::default())
				.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
				.add_plugin(RapierDebugRenderPlugin::default())
				.insert_resource(RapierConfiguration{
					gravity: Vec3::new(0.,-9.8,0.),
					..default()
				})
				.add_startup_system(utility::surrender_focus)
				.add_startup_system(spawn)
				.run();
	}
}

pub fn spawn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let cell_width = 1.;

	maze_floor::spawn(
		&mut commands,
		&mut meshes,
		&mut materials,
		cell_width,
		0.2,
		10,
		10,
	);
	ball::spawn(&mut commands, &mut meshes, &mut materials, cell_width);
}
