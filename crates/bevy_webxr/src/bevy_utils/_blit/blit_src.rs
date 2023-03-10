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
use web_sys::*;
// use bevy::window::Windows;
use crate::*;
use wgpu::Extent3d;
use wgpu::TextureDimension;
use wgpu::TextureFormat;



#[derive(Resource, Clone)]
pub struct BlitSrc {
	pub handle: Handle<Image>,
	pub width: u32,
	pub height: u32,
}

//done before frame
pub fn update_src_image_size(app: &mut App, frame: &XrFrame) {
	let mut src_image =
		app.world.get_resource_mut::<bevy_utils::BlitSrc>().unwrap();
	let gl_layer = frame.session().render_state().base_layer().unwrap();

	src_image.width = gl_layer.framebuffer_width();
	src_image.height = gl_layer.framebuffer_height();
}

pub fn resize_src_image(
	mut images: ResMut<Assets<Image>>,
	src_image: Res<BlitSrc>,
) {
	let width = src_image.width;
	let height = src_image.height;

	let mut image = images.get_mut(&src_image.handle).unwrap();
	let curr_width = image.size().x as u32;
	let curr_height = image.size().y as u32;

	if curr_width == width && curr_height == height {
		return;
	}
	// log!(
	// 	"resizing src image from {curr_width}, {curr_height} to {}, {}",
	// 	width,
	// 	height
	// );

	// TODO results in memory leak
	// image.resize(Extent3d {
	// 	width,
	// 	height,
	// 	..default()
	// });
}
