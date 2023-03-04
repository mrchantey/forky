use bevy::prelude::*;
pub const TAU: f32 = std::f32::consts::TAU;

pub struct SimplePlugin;

impl Plugin for SimplePlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Speed(0.25))
			.add_startup_system(setup)
			.add_system(rotate)
			.add_plugins(DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					// width: 1000.,
					// height: 800.,
					// title: "Forky".to_string(),
					// decorations: true,
					// cursor_visible: true,
					// cursor_grab_mode: bevy::window::CursorGrabMode::None,
					// resizable: true,
					// // return:true,
					// // winit
					// present_mode: PresentMode::AutoVsync,
					// position: WindowPosition::At(Vec2::new(-1440., 0.)),
					..Default::default()
				}),
				..Default::default()
			}));
	}
}


#[derive(Component)]
pub struct Shape;
#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct Speed(f32);
impl Speed {
	pub fn new(speed: f32) -> Speed { Speed(speed) }
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
			transform: Transform::from_xyz(0.0, 0.5, 0.0),
			..default()
		},
		Shape,
	));
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(-2.0, 2.5, 5.0)
				.looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		MainCamera,
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
