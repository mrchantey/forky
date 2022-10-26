use bevy::{
	prelude::*,
	window::{PresentMode, WindowDescriptor},
};
use bevy_inspector_egui::WorldInspectorPlugin;
use std::f32::consts::PI;

// pub const
pub struct DuckPlugin;


const CLEAR_COLOR: Color = Color::hsl(0.3 * 360., 1., 0.8);

impl Plugin for DuckPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(CLEAR_COLOR))
			.add_startup_system(spawn_basic_scene)
			.add_startup_system(spawn_camera)
			.add_system(rotate_cube);
	}
}

const TAU: f32 = PI * 2.;

fn spawn_camera(mut commands: Commands) {
	let mut transform = Transform::from_xyz(0., 1., -10.);
	transform.rotate_local_y(TAU / 2.);
	commands.spawn().insert_bundle(Camera3dBundle {
		// transform::Transform
		transform,
		// transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}

#[derive(Component)]
struct CompanionCube;

fn spawn_basic_scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
			material: materials.add(Color::rgb(0., 1., 1.).into()),
			transform: Transform::from_xyz(0., 1., 0.),
			..default()
		})
		.insert(CompanionCube);

	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Quad {
			size: Vec2::new(2., 2.),
			..default()
		})),
		transform: Transform::from_xyz(0., 1., 0.).with_rotation(Quat::from_rotation_y(TAU / 2.)),
		// mesh: meshes.add(Mesh::from(shape::Plane { size: 10. })),
		material: materials.add(Color::rgb(0.2, 1., 0.2).into()),
		..default()
	});

	commands.spawn_bundle(PointLightBundle {
		transform: Transform::from_xyz(1., 3., -3.),
		..default()
	});
}

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<CompanionCube>>) {
	for mut transform in &mut query {
		transform.rotate_y(TAU * 0.1 * time.delta_seconds());
	}
}


pub fn say_hi() {
	println!("hi");
}

pub fn run() {
	App::new()
		.insert_resource(WindowDescriptor {
			width: 500.,
			height: 500.,
			title: "MagPie".to_string(),
			decorations: true,
			cursor_visible: true,
			cursor_locked: false,
			present_mode: PresentMode::AutoVsync,
			position: WindowPosition::At(Vec2::new(-1440., 0.)),
			..Default::default()
		})
		.add_plugin(DuckPlugin)
		.add_plugins(DefaultPlugins)
		.add_plugin(WorldInspectorPlugin::new())
		.run();
}
