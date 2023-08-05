use bevy::prelude::*;
use forky_play::materials::UnlitMaterial;
use forky_play::*;

fn main() {
	App::new()
		.add_plugins(plugins::ForkyDebugPlugin::default())
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<UnlitMaterial>>,
) {
	commands.spawn(MaterialMeshBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		material: materials.add(UnlitMaterial {
			color: Color::GREEN,
			// color_texture: Some(asset_server.load("branding/icon.png")),
			alpha_mode: AlphaMode::Blend,
		}),
		..default()
	});
}
