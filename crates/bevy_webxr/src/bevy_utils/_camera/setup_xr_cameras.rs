use crate::*;
use bevy::{
	core_pipeline::clear_color::ClearColorConfig,
	prelude::*,
	render::{
		camera::{
			CameraProjection, ManualTextureViews, RenderTarget, Viewport,
		},
		extract_resource::ExtractResource,
		primitives::Frustum,
	},
};
use derive_deref::{Deref, DerefMut};
use web_sys::*;

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
	views: NonSend<Vec<bevy_utils::BevyXrView>>,
	texture_id: Res<FramebufferTextureViewId>,
	frame: NonSend<web_sys::XrFrame>,
) {
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
			.remove::<Projection>()
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
