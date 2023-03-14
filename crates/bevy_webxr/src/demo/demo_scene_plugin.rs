use std::f32::consts::TAU;

use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	log::LogPlugin,
	prelude::*,
	window::{CompositeAlphaMode, WindowResolution},
	winit::WinitPlugin,
};

use crate::{bevy_utils::UnlitMaterial, *};

const SCENE_Z: f32 = -5.0;

pub struct DemoScenePlugin;
// #[rustfmt::skip]
impl Plugin for DemoScenePlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(Speed(0.25))
			// .insert_resource(ClearColor(Color::rgba(0., 0., 0., 0.)))
			.add_startup_system(spawn_cube)
			.add_startup_system(spawn_ground)
			.add_startup_system(spawn_logo)
			// .add_startup_system(spawn_cubes)
			.add_startup_system(spawn_camera)
			.add_startup_system(spawn_lights)
			.add_system(rotate)
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

pub fn spawn_camera(mut commands: Commands) {
	let dist = 0.5;
	commands.spawn((
		Camera3dBundle {
			// camera_3d: Camera3d {
			// 	clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 0.0)),
			// 	..default()
			// },
			// transform: Transform::from_xyz(0., 0., 0.),
			transform: Transform::from_xyz(-dist, dist, dist)
				.looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		MainCamera,
	));
}


pub fn spawn_ground(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<bevy_utils::UnlitTextureMaterial>>,
	asset_server: Res<AssetServer>,
) {
	let size: f32 = 10.;
	commands.spawn(MaterialMeshBundle {
		mesh: meshes.add(Mesh::from(shape::Quad {
			size: Vec2::new(size, size),
			..default()
		})),
		material: materials.add(bevy_utils::UnlitTextureMaterial {
			color: Color::CYAN,
			tiling: Vec4::new(size, size, 0., 0.),
			color_texture: Some(asset_server.load("textures/grid_dark.png")),
			..default()
		}),
		transform: Transform::from_xyz(0., -1., 0.)
			.with_rotation(Quat::from_rotation_x(TAU * -0.25)),
		..default()
	});
}
pub fn spawn_logo(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<bevy_utils::UnlitTextureMaterial>>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(MaterialMeshBundle {
		mesh: meshes.add(Mesh::from(shape::Quad {
			size: Vec2::new(3., 1.),
			..default()
		})),
		material: materials.add(bevy_utils::UnlitTextureMaterial {
			color_texture: Some(asset_server.load("textures/logo_text.png")),
			..default()
		}),
		transform: Transform::from_xyz(0., 0., -5.),
		..default()
	});
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
pub fn spawn_cube(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<bevy_utils::UnlitMaterial>>,
) {
	commands.spawn((
		MaterialMeshBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 0.05 })),
			material: materials.add(bevy_utils::UnlitMaterial {
				color: Color::CYAN,
				..default()
			}),
			transform: Transform::from_xyz(0.0, 0., -0.3),
			..default()
		},
		Shape,
	));
}

pub fn spawn_cube_grid(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<bevy_utils::UnlitMaterial>>,
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
				MaterialMeshBundle {
					mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
					material: materials.add(bevy_utils::UnlitMaterial {
						color: Color::CYAN,
						..default()
					}),
					transform: Transform::from_xyz(x, y, SCENE_Z),
					..Default::default()
				},
				Shape,
			));
		}
	}
}
pub fn rotate(
	mut query: Query<&mut Transform, With<Shape>>,
	time: Res<Time>,
	speed: Res<Speed>,
) {
	for mut transform in &mut query {
		transform.rotate_y(time.delta_seconds() * speed.0 * TAU);
		transform.rotate_x(time.delta_seconds() * speed.0 * TAU * 0.5);
	}
}

// resize_constraints: WindowResizeConstraints {
// resolution: WindowResolution::from((100., 100.)),
// resolution: WindowResolution::from((1000., 500.)),
// resolution: WindowResolution::from((1280., 720.)),
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
