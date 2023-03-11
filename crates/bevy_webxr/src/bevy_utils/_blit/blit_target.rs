use std::f32::consts::TAU;
use std::sync::MutexGuard;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::ecs::query;
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::extract_component::ExtractComponent;
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

pub fn insert_blit_target(
	app: &mut App,
	gl_layer: &XrWebGlLayer,
) -> Handle<Image> {
	let device = app.world.resource::<RenderDevice>().wgpu_device();
	// let width = gl_layer.framebuffer_width();
	// let height = gl_layer.framebuffer_height();
	let width = 1000;
	let height = 500;
	// let width = 8145;
	// let height = 4073;

	let mut src_image = create_image(width, height);
	src_image.texture_descriptor.usage = wgpu::TextureUsages::TEXTURE_BINDING
		| wgpu::TextureUsages::COPY_DST
		| wgpu::TextureUsages::RENDER_ATTACHMENT
		| wgpu::TextureUsages::COPY_SRC;

	let dest_tex =
		crate::wgpu_utils::create_framebuffer_texture(device, gl_layer);
	let mut images = app.world.get_resource_mut::<Assets<Image>>().unwrap();
	let src_image_handle = images.add(src_image);

	app.world.insert_resource(bevy_utils::BlitSrc {
		handle: src_image_handle.clone(),
	});

	let blit_target = BlitTarget {
		src: src_image_handle.clone(),
		dest: dest_tex,
		width,
		height,
		frame_count: 0,
	};

	let mut render_app = app.get_sub_app_mut(RenderApp).unwrap();
	render_app.insert_resource(blit_target);
	return src_image_handle;
}

impl BlitTarget {
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
