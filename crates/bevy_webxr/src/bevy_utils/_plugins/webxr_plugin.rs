use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::RenderApp;

use crate::*;
use web_sys::*;

pub struct WebXrPlugin {
	/// Used to index a texture view for the xr camera. This can be any u32.
	pub framebuffer_id: u32,
	/// Session Mode, default is ImmersiveVr
	pub session_mode: web_sys::XrSessionMode,
	/// If None, a suitable reference space type for the session mode will be used.
	pub reference_space_type: Option<web_sys::XrReferenceSpaceType>,
}

impl Default for WebXrPlugin {
	fn default() -> Self {
		Self {
			framebuffer_id: Default::default(),
			session_mode: web_sys::XrSessionMode::ImmersiveVr,
			reference_space_type: None,
		}
	}
}

#[rustfmt::skip]
impl Plugin for WebXrPlugin {
	fn build(&self, app: &mut App) {
		app.__()
		.add_plugin(bevy_utils::WebXrBasePlugin)
		.add_plugin(bevy_utils::InvertPlugin)
		.add_plugin(ExtractResourcePlugin::<bevy_utils::FramebufferTextureViewId>::default())
		.add_event::<bevy_utils::InputSourceAdded>()
		.add_event::<bevy_utils::InputSourceRemoved>()
		// .add_event::<bevy_utils::InputSourceRemovedEvent>()
		.insert_resource(bevy_utils::FramebufferTextureViewId(self.framebuffer_id))
		.insert_non_send_resource(self.session_mode)
		.insert_non_send_resource(xr_utils::unwrap_reference_space_type(&self.session_mode, self.reference_space_type))
			.add_system(bevy_utils::update_xr_resources
				.in_base_set(CoreSet::PreUpdate)
			)
			
			//Input Sources
			.insert_resource(bevy_utils::InputSourceAssetLookup::default())
			.add_system(bevy_utils::remove_input_sources
				.in_base_set(CoreSet::PreUpdate)
				.after(bevy_utils::update_xr_resources)
			)
			.add_system(bevy_utils::rebuild_input_sources
				.in_base_set(CoreSet::PreUpdate)
				.after(bevy_utils::update_xr_resources)
			)
			.add_system(bevy_utils::update_input_sources
				// .in_base_set(CoreSet::PreUpdate)
				.after(bevy_utils::rebuild_input_sources)
			)
			
			//Cameras
			.add_system(bevy_utils::remove_xr_cameras
				.in_base_set(CoreSet::PreUpdate)
				.run_if(bevy_utils::cameras_need_rebuild)
				.after(bevy_utils::update_xr_resources)
			)
			.add_system(bevy_utils::setup_xr_cameras
				.in_base_set(CoreSet::PreUpdate)
				.run_if(bevy_utils::cameras_need_rebuild)
				.after(bevy_utils::remove_xr_cameras)
			)
			.add_system(bevy_utils::update_xr_cameras
				// .in_base_set(CoreSet::PreUpdate)
				.after(bevy_utils::setup_xr_cameras)
			)
			.add_system(bevy_utils::update_manual_texture_views)

			//Misc

			.add_startup_system(xr_utils::set_canvas_size)
			.__();
	}
}
