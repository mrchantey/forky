use bevy::prelude::*;
use forky_core::math::TAU;

use crate::utility::WorldInspectorPlugin;


pub struct SimplePlugin;

impl Plugin for SimplePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(WorldInspectorPlugin)
			.insert_resource(Speed(0.25))
			.add_startup_system(spawn_default_camera)
			.add_startup_system(spawn_cube)
			.add_system(rotate);
	}
}


#[derive(Component)]
struct Shape;

#[derive(Resource)]
pub struct Speed(f32);
impl Speed {
	pub fn new(speed: f32) -> Speed { Speed(speed) }
}

pub fn spawn_default_camera(
	mut commands: Commands,
) {
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0., 0., 5.0)
			// transform: Transform::from_xyz(-2.0, 2.5, 5.0)
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}

pub fn spawn_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			..default()
		},
		Shape,
	));
}

fn rotate(
	mut query: Query<&mut Transform, With<Shape>>,
	time: Res<Time>,
	speed: Res<Speed>,
) {
	for mut transform in &mut query {
		transform.rotate_y(time.delta_seconds() * speed.0 * TAU);
	}
}
