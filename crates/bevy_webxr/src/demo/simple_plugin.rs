use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	log::LogPlugin,
	prelude::*,
	window::{CompositeAlphaMode, WindowResolution},
	winit::WinitPlugin,
};

// use ColorMaterial: = crate::bevy_utils::color_material::ColorMaterial;

use crate::{bevy_utils::MyColorMaterial, *};
pub const TAU: f32 = std::f32::consts::TAU;

pub struct SimplePlugin;

impl Plugin for SimplePlugin {
	fn build(&self, app: &mut App) {
		// xr_utils::create_canvas(xr_utils::BEVY_CANVAS_ID).unwrap();
		xr_utils::create_default_canvas().unwrap();

		app.__()
			.add_plugins(
				DefaultPlugins
					.set(WindowPlugin {
						primary_window: Some(Window {
							// resolution: WindowResolution::from((100., 100.)),
							// resolution: WindowResolution::from((1000., 500.)),
							// resolution: WindowResolution::from((1280., 720.)),
							title: "Bevy WebXR Demo".into(),
							canvas: Some(xr_utils::BEVY_CANVAS_QUERY.into()),
							// resize_constraints: WindowResizeConstraints {
							// 	min_width: 100.,
							// 	min_height: 100.,
							// 	max_width: 10.,
							// 	max_height: 100.,
							// },
							// fit_canvas_to_parent:true,
							// transparent: true,
							// composite_alpha_mode:CompositeAlphaMode::Opaque,
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
			)
			.insert_resource(Speed(0.25))
			// .insert_resource(ClearColor(Color::rgba(0., 0., 0., 0.)))
			.add_startup_system(setup_cube_unlit)
			// .add_startup_system(setup_cubes)
			.add_startup_system(setup_camera)
			.add_startup_system(spawn_lights)
			.add_system(rotate)
			.add_plugin(MaterialPlugin::<bevy_utils::MyColorMaterial>::default())
			.__();
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
			// priority: -1,
			// camera_3d: Camera3d {
			// 	clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 0.0)),
			// 	..default()
			// },
			// transform: Transform::from_xyz(-2.0, 2.5, 5.0)
			transform: Transform::from_xyz(0., 0., 0.),
			// .looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		MainCamera,
	));
}

const SCENE_Z: f32 = -5.0;

fn setup_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			material: materials.add(Color::WHITE.into()),
			transform: Transform::from_xyz(0.0, 0., SCENE_Z),
			..default()
		},
		Shape,
	));
}
fn setup_cube_unlit(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<MyColorMaterial>>,
	// mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// let material = Color::WHITE.into();
	let material = bevy_utils::MyColorMaterial {
		color: Color::CYAN,
		..default()
	};

	commands.spawn((
		MaterialMeshBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			material: materials.add(material),
			transform: Transform::from_xyz(0.0, 0., SCENE_Z),
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
	let num_cubes = 3;
	let min_num_cubes = num_cubes / 2 * -1;
	let max_num_cubes = num_cubes / 2 + 1;
	for i in min_num_cubes..max_num_cubes {
		for j in min_num_cubes..max_num_cubes {
			let x = i as f32 * spacing;
			let y = j as f32 * spacing;
			commands.spawn((
				PbrBundle {
					mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
					material: materials.add(Color::WHITE.into()),
					transform: Transform::from_xyz(x, y, SCENE_Z),
					..Default::default()
				},
				Shape,
			));
		}
	}
}
pub fn spawn_lights(mut commands: Commands) {
	commands.insert_resource(AmbientLight {
		color: Color::WHITE,
		brightness: 1.,
	});
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(-5., 5., 3.),
		point_light: PointLight {
			intensity: 10000., //10x for xr?
			color: Color::FUCHSIA,
			shadows_enabled: false,
			..default()
		},
		..default()
	});
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(3., 5., -5.),
		point_light: PointLight {
			intensity: 10000., //10x for xr?
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
		transform.rotate_x(time.delta_seconds() * speed.0 * TAU * 0.5);
	}
}


// pub struct SimpleNoWinitPlugin;

// impl Plugin for SimpleNoWinitPlugin {
// 	fn build(&self, app: &mut App) {
// 		xr_utils::create_canvas().unwrap();

// 		app.insert_resource(Speed(0.25))
// 			.insert_resource(ClearColor(Color::rgba(0., 0., 0., 0.)))
// 			// .add_startup_system(setup_cube)
// 			.add_startup_system(setup_cubes)
// 			.add_startup_system(setup_camera)
// 			.add_startup_system(spawn_lights)
// 			.add_system(rotate)
// 			.add_plugins(
// 				DefaultPlugins
// 					.set(LogPlugin {
// 						level: bevy::log::Level::WARN,
// 						..Default::default()
// 					})
// 					.build()
// 					.disable::<WinitPlugin>(),
// 			);
// 	}
// }
