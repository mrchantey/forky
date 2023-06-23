use crate::*;
use bevy::prelude::*;
use bevy::render::camera::{ManualTextureView, ManualTextureViews};
use bevy::render::renderer::RenderDevice;

pub fn update_manual_texture_views(
	gl_layer: NonSend<web_sys::XrWebGlLayer>,
	render_device: Res<RenderDevice>,
	texture_id: Res<bevy_utils::FramebufferTextureViewId>,
	mut manual_tex_view: ResMut<ManualTextureViews>,
) {
	let dest_texture = xr_utils::create_framebuffer_texture(
		render_device.wgpu_device(),
		&gl_layer,
	);

	let size =
		UVec2::new(gl_layer.framebuffer_width(), gl_layer.framebuffer_height());

	let descriptor = wgpu::TextureViewDescriptor::default();
	let view = dest_texture.create_view(&descriptor);

	manual_tex_view.insert(
		**texture_id,
		ManualTextureView::with_default_format(view.into(), size),
	);
}
