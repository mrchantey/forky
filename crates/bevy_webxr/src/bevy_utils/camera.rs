use crate::*;
use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	prelude::*,
	render::camera::{RenderTarget, Viewport},
};
use web_sys::XrSessionMode;

use super::BlitSrc;

#[derive(Component)]
pub struct LeftCamera;

#[derive(Component)]
pub struct RightCamera;
#[derive(Component)]
pub struct XrCamera;


fn spawn_clear_camera() {}

pub fn setup_xr_cameras(
	mut commands: Commands,
	mode: Res<bevy_utils::SessionMode>,
	images: Res<Assets<Image>>,
	src_image: Res<BlitSrc>,
) {
	// let image = images.get(&src_image.0).unwrap();

	let width = src_image.width;
	let h_width = width / 2;
	let height = src_image.height;

	let (num_cameras, camera_width) = if mode.0 == XrSessionMode::Inline {
		(1, width)
	} else {
		(2, h_width)
	};

	//clear hack
	commands.spawn(Camera3dBundle {
		camera_3d: Camera3d {
			// clear_color: ClearColorConfig::Default,
			clear_color: ClearColorConfig::Custom(Color::rgb(1.0, 1.0, 1.0)),
			..default()
		},
		camera: Camera {
			order: 2,
			target: RenderTarget::Image(src_image.handle.clone()),
			viewport: Some(Viewport {
				physical_position: UVec2::new(0, 0),
				physical_size: UVec2::new(1, 1),
				..default()
			}),
			..default()
		},
		..default()
	});


	for i in 0..num_cameras {
		let pos_x_offset = 0.5;
		let (uv_x, pos_x) = if i == 0 {
			(0, -pos_x_offset)
		} else {
			(h_width, pos_x_offset)
		};
		// log!("camera width: {camera_width}, uv_x: {uv_x}, pos_x: {pos_x}");
		let mut entity = commands.spawn(Camera3dBundle {
			transform: Transform::from_xyz(pos_x, 0., 5.0), //inital, will be overridden
			camera_3d: Camera3d {
				clear_color: ClearColorConfig::None, //split screen
				..default()
			},
			camera: Camera {
				order: i,
				// render before the "main pass" camera
				// order: -1,
				target: RenderTarget::Image(src_image.handle.clone()),
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
		entity.insert(XrCamera);
		if i == 0 {
			entity.insert(LeftCamera);
		} else {
			entity.insert(RightCamera);
		}
	}
}
