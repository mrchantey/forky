use std::f32::consts::TAU;
use std::sync::MutexGuard;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use bevy::render::view::WindowSurfaces;
use bevy::render::RenderApp;
// use bevy::window::Windows;
use wgpu::Extent3d;
use wgpu::TextureDimension;
use wgpu::TextureFormat;

use super::create_image;
use super::BlitSource;

pub fn create_blit_source(app: &mut App) -> BlitSource {
	let device = app.world.resource::<RenderDevice>().wgpu_device();

	let mut src_image = create_image(500, 500);
	src_image.texture_descriptor.usage = wgpu::TextureUsages::TEXTURE_BINDING
		| wgpu::TextureUsages::COPY_DST
		| wgpu::TextureUsages::RENDER_ATTACHMENT
		| wgpu::TextureUsages::COPY_SRC;

	let mut images = app.world.get_resource_mut::<Assets<Image>>().unwrap();
	let src_image_handle = images.add(src_image).clone();

	app.world.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0., 0., 5.0)
			.looking_at(Vec3::ZERO, Vec3::Y),
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

	BlitSource {
		src: src_image_handle,
	}
}
