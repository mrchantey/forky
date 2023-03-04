use std::f32::consts::TAU;
use std::sync::MutexGuard;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use bevy::render::view::WindowSurfaces;
use bevy::render::RenderApp;
use web_sys::XrWebGlLayer;
// use bevy::window::Windows;
use wgpu::Extent3d;
use wgpu::TextureDimension;
use wgpu::TextureFormat;

use crate::core::create_framebuffer_texture;

use super::*;

pub struct BlitGraphPlugin;


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
		wgpu::TextureFormat::Rgba8Unorm
		// TextureFormat::bevy_default(),
		// TextureFormat::Rgba8UnormSrgb,
	)
	// dest_image.texture_descriptor.sample_count = 1;
}

pub fn create_blit_target(
	app: &mut MutexGuard<App>,
	gl_layer: &XrWebGlLayer,
) -> BlitTarget {
	let device = app.world.resource::<RenderDevice>().wgpu_device();
	let width = gl_layer.framebuffer_width();
	let height = gl_layer.framebuffer_height();

	let dest_tex = create_framebuffer_texture(device, gl_layer);


	let mut src_image = create_image(width, height);
	src_image.texture_descriptor.usage = wgpu::TextureUsages::TEXTURE_BINDING
		| wgpu::TextureUsages::COPY_DST
		| wgpu::TextureUsages::RENDER_ATTACHMENT
		| wgpu::TextureUsages::COPY_SRC;

	let mut images = app.world.get_resource_mut::<Assets<Image>>().unwrap();
	let src_image_handle = images.add(src_image).clone();

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

	BlitTarget {
		src: src_image_handle,
		dest: dest_tex,
		width,
		height,
	}
}
