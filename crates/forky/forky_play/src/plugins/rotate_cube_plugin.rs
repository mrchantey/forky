use crate::*;
use bevy::prelude::*;
use forky_core::math::TAU;

pub struct RotateCubePlugin;

impl Plugin for RotateCubePlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(Speed(0.25))
			.add_systems(Startup, spawn_cube)
			.add_systems(Update, rotate)
			.__();
	}
}


#[derive(Component)]
struct Shape;

#[derive(Resource)]
pub struct Speed(f32);
impl Speed {
	pub fn new(speed: f32) -> Speed { Speed(speed) }
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
	time: Res<Time<Real>>,
	speed: Res<Speed>,
) {
	for mut transform in &mut query {
		transform.rotate_y(time.delta_seconds() * speed.0 * TAU);
	}
}
