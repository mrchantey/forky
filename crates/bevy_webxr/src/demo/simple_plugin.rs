use bevy::{log::LogPlugin, prelude::*, winit::WinitPlugin};

use crate::xr_utils;
pub const TAU: f32 = std::f32::consts::TAU;


pub struct SimplePlugin;

impl Plugin for SimplePlugin {
	fn build(&self, app: &mut App) {
		xr_utils::create_canvas().unwrap();

		app.insert_resource(Speed(0.25))
			.insert_resource(ClearColor(Color::rgba(0., 0., 0., 0.)))
			// .add_startup_system(setup_cube)
			.add_startup_system(setup_cubes)
			.add_startup_system(setup_camera)
			.add_startup_system(spawn_lights)
			.add_system(rotate)
			.add_plugins(
				DefaultPlugins
					.set(WindowPlugin {
						primary_window: Some(Window {
							// width: 1000.,
							// height: 800.,
							title: "Bevy WebXR Demo".into(),
							canvas: Some(xr_utils::CANVAS_QUERY.into()),
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
					})
					.set(LogPlugin {
						// filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
						level: bevy::log::Level::WARN,
						..Default::default()
					}),
			);
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

fn setup_camera(mut commands: Commands) {
	commands.spawn((
		Camera3dBundle {
			// transform: Transform::from_xyz(-2.0, 2.5, 5.0)
			transform: Transform::from_xyz(0., 0., 5.0)
				.looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		MainCamera,
	));
}

fn setup_cube(
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
}
fn setup_cubes(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let spacing = 1.5;
	for i in -5..6 {
		for j in -5..6 {
			let x = i as f32 * spacing;
			let y = j as f32 * spacing;
			commands.spawn((
				PbrBundle {
					mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
					material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
					transform: Transform::from_xyz(x, y, 0.0),
					..Default::default()
				},
				Shape,
			));
		}
	}
}
pub fn spawn_lights(mut commands: Commands) {
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(-5., 5., 3.),
		point_light: PointLight {
			intensity: 1000.,
			color: Color::FUCHSIA,
			shadows_enabled: true,
			..default()
		},
		..default()
	});
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(3., 5., -5.),
		point_light: PointLight {
			intensity: 1000.,
			shadows_enabled: true,
			color: Color::CYAN,
			..default()
		},
		..default()
	});
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
