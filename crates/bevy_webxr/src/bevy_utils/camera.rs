use crate::*;
use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	prelude::*,
	render::camera::{RenderTarget, Viewport},
};
use web_sys::*;

use super::BlitSrc;

#[derive(Component)]
pub struct LeftCamera;

#[derive(Component)]
pub struct RightCamera;
#[derive(Component)]
pub struct XrCamera {
	pub index: usize,
}


fn spawn_clear_camera() {}

pub fn setup_xr_cameras(
	mut commands: Commands,
	mode: NonSend<XrSessionMode>,
	images: Res<Assets<Image>>,
	src_image: Res<BlitSrc>,
) {
	let image = images.get(&src_image.handle).unwrap();

	let width = image.size().x as u32;
	let h_width = width / 2;
	let height = image.size().y as u32;

	let (num_cameras, camera_width) = if *mode == XrSessionMode::Inline {
		(1, width)
	} else {
		(2, h_width)
	};

	for i in 0..num_cameras {
		let pos_x_offset = 0.5;
		//the left camera clears entire target
		let (uv_x, pos_x, clear_color, order) = if i == 0 {
			(0, -pos_x_offset, ClearColorConfig::None, 1)
		// (0, -pos_x_offset, ClearColorConfig::default(), 1)
		} else {
			(h_width, pos_x_offset, ClearColorConfig::default(), 0)
		};
		// log!("camera width: {camera_width}, uv_x: {uv_x}, pos_x: {pos_x}");
		let mut entity = commands.spawn(Camera3dBundle {
			transform: Transform::from_xyz(pos_x, 0., 5.0), //inital, will be overridden
			camera_3d: Camera3d {
				clear_color, //split screen
				..default()
			},
			camera: Camera {
				order,
				// render before the "main pass" camera
				// order: -1,
				target: RenderTarget::TextureView(0),
				// target: RenderTarget::Image(src_image.handle.clone()),
				viewport: Some(Viewport {
					physical_position: UVec2::new(uv_x, 0),
					physical_size: UVec2::new(camera_width, height),
					..default()
				}),
				..default()
			},
			// transform: Transform::from_xyz(-2.0, 2.5, 5.0)
			..default()
		});
		entity.insert(XrCamera { index: i });
		if i == 0 {
			entity.insert(LeftCamera);
		} else {
			entity.insert(RightCamera);
		}
	}
}
