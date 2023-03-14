use crate::*;
use bevy::{
	core_pipeline::{
		clear_color::ClearColorConfig,
		tonemapping::{DebandDither, Tonemapping},
	},
	prelude::*,
	render::{
		camera::{
			CameraProjection, ManualTextureViews, RenderTarget, Viewport,
		},
		extract_resource::ExtractResource,
		primitives::Frustum,
		view::ColorGrading,
	},
};
use derive_deref::{Deref, DerefMut};
use web_sys::*;

use super::BevyXrViewLookup;

#[derive(Resource, Clone, Deref, DerefMut, ExtractResource)]
pub struct FramebufferTextureViewId(pub u32);

#[derive(Component)]
pub struct LeftCamera;

#[derive(Component)]
pub struct RightCamera;
#[derive(Component)]
pub struct XrCamera {
	pub index: usize,
}

pub fn remove_xr_cameras(
	mut commands: Commands,
	query: Query<Entity, With<XrCamera>>,
) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn setup_xr_cameras(
	mut commands: Commands,
	mode: NonSend<XrSessionMode>,
	views: Res<BevyXrViewLookup>,
	texture_id: Res<FramebufferTextureViewId>,
	frame: NonSend<web_sys::XrFrame>,
) {
	log!("setup_xr_cameras");
	for (i, view) in views.iter().enumerate() {
		let index = i;
		//the first/left camera clears entire target
		let clear_color = if i == 0 {
			// ClearColorConfig::Custom(Color::OLIVE)
			ClearColorConfig::default()
		} else {
			ClearColorConfig::None
		};
		let view_projection =
			view.projection.get_projection_matrix() * Mat4::IDENTITY.inverse();

		// let frustum = Frustum::from_view_projection(&view_projection);

		let mut entity = commands.spawn(Camera3dBundle {
			camera_3d: Camera3d {
				clear_color, //split screen
				..default()
			},
			camera: Camera {
				order: index as isize,
				target: RenderTarget::TextureView(**texture_id),
				viewport: Some(view.viewport.clone()),
				..default()
			},
			..default()
		});
		entity
			.__()
			// .insert(frustum)
			// .remove::<Tonemapping>()
			// .remove::<DebandDither>()
			// .remove::<ColorGrading>()
			.remove::<Projection>()
			.insert(view.clone())
			.insert(view.projection.clone())
			.insert(XrCamera { index })
			.__();
		if i == 0 {
			entity.insert(LeftCamera);
		} else {
			entity.insert(RightCamera);
		}
	}
}
