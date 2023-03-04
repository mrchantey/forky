use std::f32::consts::TAU;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::texture::BevyDefault;
use bevy::render::view::WindowSurfaces;
use bevy::render::RenderApp;
// use bevy::window::Windows;
use wgpu::Extent3d;
use wgpu::TextureDimension;
use wgpu::TextureFormat;

use super::*;

pub struct BlitGraphPlugin;


#[derive(Resource, Clone, Default)]
pub struct BlitImageHandle {
	pub src: Handle<Image>,
	pub dest: Handle<Image>,
	pub width: u32,
	pub height: u32,
}

fn create_image(width: u32, height: u32) -> Image {
	Image::new_fill(
		Extent3d {
			width,
			height,
			..Default::default()
		},
		TextureDimension::D2,
		// &[255u8;4],
		&[127, 255, 255, 255],
		TextureFormat::bevy_default(),
		// TextureFormat::Rgba8UnormSrgb,
	)
	// dest_image.texture_descriptor.sample_count = 1;
}


impl Plugin for BlitGraphPlugin {
	fn build(&self, app: &mut App) {
		let width = 300;
		let height = 150;

		let mut src_image = create_image(width, height);
		src_image.texture_descriptor.usage =
			wgpu::TextureUsages::TEXTURE_BINDING
				| wgpu::TextureUsages::COPY_DST
				| wgpu::TextureUsages::RENDER_ATTACHMENT
				| wgpu::TextureUsages::COPY_SRC;
		let mut dest_image = create_image(width, height);
		dest_image.texture_descriptor.usage =
			wgpu::TextureUsages::TEXTURE_BINDING
				| wgpu::TextureUsages::COPY_DST;
		let mut images = app.world.get_resource_mut::<Assets<Image>>().unwrap();
		let src_image_handle = images.add(src_image).clone();
		let dest_image_handle = images.add(dest_image).clone();

		let mut materials = app
			.world
			.get_resource_mut::<Assets<StandardMaterial>>()
			.unwrap();
		let material_handle = materials.add(StandardMaterial {
			base_color_texture: Some(dest_image_handle.clone()),
			unlit: true,
			..default()
		});
		let mut meshes = app.world.get_resource_mut::<Assets<Mesh>>().unwrap();
		let mesh_handle = meshes.add(Mesh::from(shape::Plane::default()));
		// app.world.insert_resource(BlitImageHandle {
		// 	src: dest_image_handle.clone(),
		// });
		app.world.spawn(PbrBundle {
			mesh: mesh_handle,
			material: material_handle,
			transform: Transform::from_xyz(2.0, 0.0, 0.0)
				.with_rotation(Quat::from_rotation_x(TAU * 0.25)),
			..default()
		});
		app.world.spawn(Camera3dBundle {
			transform: Transform::from_xyz(0., 0., 5.0),
			// .looking_at(Vec3::ZERO, Vec3::Y),
			camera_3d: Camera3d {
				clear_color: ClearColorConfig::Custom(Color::WHITE),
				..default()
			},
			camera: Camera {
				// render before the "main pass" camera
				order: -1,
				target: RenderTarget::Image(src_image_handle.clone()),
				..default()
			},
			// transform: Transform::from_xyz(-2.0, 2.5, 5.0)
			..default()
		});

		// app.add_system(log_image);
		let render_app = app.get_sub_app_mut(RenderApp).unwrap();
		render_app.insert_resource(BlitImageHandle {
			src: src_image_handle.clone(),
			dest: dest_image_handle.clone(),
			width,
			height,
		});
		// render_app.system

		let my_node = BlitNode::new(&mut render_app.world);
		insert_final_node(render_app, my_node, "blit_pass", BlitNode::IN_VIEW);
	}
}


// fn log_image(handle: Res<CustomImageHandle>, images: Res<Assets<Image>>) {
// 	let image = images.get(&handle.handle).unwrap();
// 	println!("image - r: {}, g: {}, b: {}, a: {}",image.data[0],image.data[1],image.data[2],image.data[3]);
// }
