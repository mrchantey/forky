use bevy::prelude::*;
use forky_play::materials::UvMaterial;
use forky_play::*;

fn main() {
	App::new()
		.add_plugin(plugins::ForkyDebugPlugin::default())
		.add_startup_system(setup)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<UvMaterial>>,
) {
	commands.spawn(MaterialMeshBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		material: materials.add(UvMaterial {
			color: Color::GREEN,
			// color_texture: Some(asset_server.load("branding/icon.png")),
			alpha_mode: AlphaMode::Blend,
		}),
		..default()
	});
}
