use std::collections::HashMap;

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
	utils::HashSet,
};
use derive_deref::{Deref, DerefMut};
use web_sys::*;

use super::BevyXrView;

#[derive(Resource, Clone, Deref, DerefMut, ExtractResource)]
pub struct FramebufferTextureViewId(pub u32);

pub fn update_views(
	mut commands: Commands,
	mut query: Query<(Entity, &mut Transform, &BevyXrView)>,
	views: NonSend<bevy_utils::ViewLookup>,
) {
	for (entity, mut transform, xr_camera) in query.iter_mut() {
		match views.get(&xr_camera.hash) {
			Some(view) => {
				let pose: bevy_utils::Pose = view.transform().into();
				transform.translation = pose.position;
				transform.rotation = pose.rotation;
			}
			None => {
				commands.entity(entity).despawn();
				continue;
			}
		};
	}
}
