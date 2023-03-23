use bevy::prelude::*;


#[derive(Component)]
pub struct BezierSlider;

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
		// Shape,
	));
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0., 0., 5.0)
			// transform: Transform::from_xyz(-2.0, 2.5, 5.0)
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}

pub fn slide(
	mut query: Query<&mut Transform, With<BezierSlider>>,
	// time: Res<Time>,
	// speed: Res<Speed>,
) {
}
