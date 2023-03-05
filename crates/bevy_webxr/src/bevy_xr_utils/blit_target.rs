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
use crate::*;
use wgpu::Extent3d;
use wgpu::TextureDimension;
use wgpu::TextureFormat;



#[derive(Resource)]
pub struct BlitTarget {
	pub src: Handle<Image>,
	pub dest: wgpu::Texture,
	pub width: u32,
	pub height: u32,
	pub frame_count: u32,
}


impl BlitTarget {
	pub fn new(app: &mut MutexGuard<App>, gl_layer: &XrWebGlLayer) -> Self {
		let device = app.world.resource::<RenderDevice>().wgpu_device();
		let width = gl_layer.framebuffer_width();
		let height = gl_layer.framebuffer_height();

		let mut src_image = create_image(width, height);
		src_image.texture_descriptor.usage =
			wgpu::TextureUsages::TEXTURE_BINDING
				| wgpu::TextureUsages::COPY_DST
				| wgpu::TextureUsages::RENDER_ATTACHMENT
				| wgpu::TextureUsages::COPY_SRC;

		let dest_tex =
			crate::wgpu_utils::create_framebuffer_texture(device, gl_layer);
		let mut images = app.world.get_resource_mut::<Assets<Image>>().unwrap();
		let src_image_handle = images.add(src_image).clone();

		app.world.spawn(Camera3dBundle {
			transform: Transform::from_xyz(0., 0., 5.0)
				.looking_at(Vec3::ZERO, Vec3::Y),
			camera_3d: Camera3d {
				// clear_color: ClearColorConfig::Custom(Color::WHITE),
				// clear_color: ClearColorConfig::Custom(Color::CYAN),
				..default()
			},
			camera: Camera {
				// order: 1,
				// render before the "main pass" camera
				// order: -1,
				target: RenderTarget::Image(src_image_handle.clone()),
				..default()
			},
			// transform: Transform::from_xyz(-2.0, 2.5, 5.0)
			..default()
		});

		Self {
			src: src_image_handle,
			dest: dest_tex,
			width,
			height,
			frame_count: 0,
		}
	}

	pub fn update_dest(&mut self, texture: wgpu::Texture) {
		self.dest = texture;
		self.frame_count += 1;
	}
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
		wgpu::TextureFormat::Rgba8Unorm, // TextureFormat::bevy_default(),
		                                 // TextureFormat::Rgba8UnormSrgb,
	)
	// dest_image.texture_descriptor.sample_count = 1;
}
