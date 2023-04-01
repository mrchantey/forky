use bevy::prelude::*;



use crate::*;


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
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum WebXrSet {
	PrePrepare,
	Prepare,
	Tracking,
}

#[rustfmt::skip]
impl Plugin for WebXrPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugin(bevy_utils::WebXrBasePlugin)
			.insert_resource(bevy_utils::FramebufferTextureViewId(self.framebuffer_id))
			.insert_non_send_resource(self.session_mode)
			.insert_non_send_resource(xr_utils::unwrap_reference_space_type(&self.session_mode, self.reference_space_type))
			.configure_sets(PreUpdate,(
				WebXrSet::PrePrepare,
				WebXrSet::Prepare, 
				WebXrSet::Tracking
				))
			.configure_set(Update,WebXrSet::Prepare.after(WebXrSet::PrePrepare))
			.configure_set(Update,WebXrSet::Tracking.after(WebXrSet::Prepare))
			//Config
			.add_startup_system(xr_utils::set_canvas_size)
			.add_system(bevy_utils::insert_gl_layer
				.in_set(WebXrSet::PrePrepare)
			)
			//Cameras
			// .add_plugin(ExtractResourcePlugin::<bevy_utils::FramebufferTextureViewId>::default())
			.add_system(bevy_utils::update_manual_texture_views
				.in_set(WebXrSet::Prepare)
			)
			.add_system(bevy_utils::insert_views
				.in_set(WebXrSet::Prepare)
			)
			.add_system(bevy_utils::create_views
				.in_set(WebXrSet::Tracking)
			)
			.add_system(bevy_utils::update_views
				.after(bevy_utils::create_views)
				.in_set(WebXrSet::Tracking)
			)
			//Input Sources
			.insert_resource(bevy_utils::InputSourceAssetLookup::default())
			.add_system(bevy_utils::insert_input_sources
				.in_set(WebXrSet::Prepare)
			)
			.add_system(bevy_utils::create_input_sources
				.in_set(WebXrSet::Tracking)
			)
			.add_system(bevy_utils::update_input_sources
				.after(bevy_utils::create_input_sources)
				.in_set(WebXrSet::Tracking)
			)
			.__();
	}
}
