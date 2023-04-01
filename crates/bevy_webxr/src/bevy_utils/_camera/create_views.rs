use crate::*;
use bevy::{
	core_pipeline::{
		clear_color::ClearColorConfig,
	},
	prelude::*,
	render::{
		camera::{
			RenderTarget,
		},
	},
	utils::HashSet,
};
use web_sys::*;
use super::{BevyXrView, FramebufferTextureViewId};


pub fn create_views(
	mut commands: Commands,
	gl_layer: NonSend<XrWebGlLayer>,
	texture_id: Res<FramebufferTextureViewId>,
	query: Query<(Entity, &mut Transform, &BevyXrView)>,
	views: NonSend<bevy_utils::ViewLookup>,
) {
	let spawned_views = query
		.iter()
		.map(|(_, _, xr_view)| xr_view.hash)
		.collect::<HashSet<_>>();

	for (i, view) in views.iter() {
		if spawned_views.contains(i) {
			continue;
		}
		// log!("creating camera for hash: {i}");
		let xr_camera = BevyXrView::new(view, &gl_layer);
		let is_left = xr_camera.viewport.physical_position.x == 0;
		//the first/left camera clears entire target
		let clear_color = if is_left {
			ClearColorConfig::default()
		} else {
			ClearColorConfig::None
		};
		let order = if is_left { 0 } else { 1 };

		let pose: bevy_utils::Pose = view.transform().into();
		let mut entity = commands.spawn(Camera3dBundle {
			camera_3d: Camera3d {
				clear_color, //split screen
				..default()
			},
			camera: Camera {
				order,
				target: RenderTarget::TextureView(**texture_id),
				viewport: Some(xr_camera.viewport.clone()),
				..default()
			},
			transform: pose.into(),
			..default()
		});
		entity
			.__()
			.remove::<Projection>()
			.insert(xr_camera.clone())
			.insert(xr_camera.projection.clone())
			.__();
		bevy_utils::insert_view_handedness(
			&mut entity,
			xr_camera.viewport,
			views.len(),
		);
	}
}
