use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_play::maze::*;
use forky_play::utility::surrender_focus;
use forky_play::*;
use sweet::*;

sweet! {
	it skip "works" {
		App::new()
			.add_plugins(plugins::ForkyFullPlugin::default())
			.add_systems(Startup, surrender_focus)
			.insert_resource(RapierConfiguration::with_gravity_scalar(10.))
			.add_event::<RespawnEvent>()
			.add_event::<DespawnEvent>()
			.add_systems(Update,maze_3d::respawn)
			.add_systems(Update,maze_3d::despawn)
			.add_systems(Update,ball::respawn)
			.add_systems(Update,ball::despawn_on_ball_fall)
			.add_systems(Startup, spawn)
			.run();
	}
}

pub fn spawn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let _obj = commands
		.spawn(PbrBundle {
			transform: Transform::from_xyz(0., -0.1, 0.)
				.with_scale_xyz(10., 0.2, 10.)
				.with_rotation_x(0.1),
			mesh: meshes.add(Mesh::from(Cuboid::default())),
			material: materials.from_white(),
			..default()
		})
		.insert(Collider::cuboid(0.5, 0.5, 0.5))
		.id();
}
