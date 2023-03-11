use crate::*;
use anyhow::Result;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::RenderApp;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;


pub async fn init_xr_render(
	app: &mut App,
) -> Result<(XrSession, XrReferenceSpace), JsValue> {
	let gl = xr_utils::create_webgl_context(true)?;
	// xr_utils::clear_canvas(&gl)?;
	// let mode = web_sys::XrSessionMode::Inline;
	let mode = web_sys::XrSessionMode::ImmersiveVr;
	app.insert_non_send_resource(mode);
	let session = xr_utils::create_xr_session_with_mode(&gl, mode).await?;

	let gl_layer = xr_utils::create_xr_gl_layer(&session, &gl).await?;
	bevy_utils::insert_blit_target(app, &gl_layer);
	let reference_space =
		xr_utils::get_reference_space(&session, &mode).await?;

	return Ok((session, reference_space));
}

pub fn update_framebuffer_texture(
	render_device: Res<RenderDevice>,
	mut blit_target: ResMut<bevy_utils::BlitTarget>,
	frame: NonSend<XrFrame>,
) {
	let gl_layer = frame.session().render_state().base_layer().unwrap();
	let dest_texture = wgpu_utils::create_framebuffer_texture(
		&render_device.wgpu_device(),
		&gl_layer,
	);
	blit_target.update_dest(dest_texture);
}
