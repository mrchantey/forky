use crate::*;
use anyhow::Result;
use bevy::prelude::*;
use bevy::render::RenderApp;
// use bevy::render::render_phase::AddRenderCommand;
// use wgpu::*;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use wgpu::{Operations, RenderPassColorAttachment, RenderPassDescriptor};
// use std::cell::RefCell;
// use std::rc::Rc;
// use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::MutexGuard;
// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;

// use wgpu::*;

//deleteme, copy of blit node



// pub fn render_xr_frame(
// 	mut app: MutexGuard<App>,
// 	frame: &XrFrame,
// ) {
// 	let session = frame.session();
// 	let gl_layer = session.render_state().base_layer().unwrap();

// 	let device = render_app.world.resource::<RenderDevice>().wgpu_device();
// 	let queue = render_app.world.resource::<RenderQueue>();

// 	let mut render_app = app.get_sub_app_mut(RenderApp).unwrap();
// 	let dest_tex =
// 		wgpu_utils::create_framebuffer_texture(device, &gl_layer);
// 	// let blit_target = render_app.world.resource::<BlitTarget>();
// 	// // blit_target.dest = dest_tex;
// 	// render_app.world.insert_resource(BlitTarget {
// 	// 	src: blit_target.src.clone(),
// 	// 	dest: dest_tex,
// 	// 	width: blit_target.width,
// 	// 	height: blit_target.height,
// 	// });
// 	let mut encoder =
// 		device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
// 			label: Some("XR Blit Encoder"),
// 		});

// 	let dest_view =
// 		&dest_tex.create_view(&wgpu::TextureViewDescriptor::default());

// 	let pass = encoder.begin_render_pass(&RenderPassDescriptor {
// 		label: Some("Clear Pass"),
// 		color_attachments: &[Some(RenderPassColorAttachment {
// 			view: dest_view,
// 			// resolve_target: Some(dest_view),
// 			resolve_target: None,
// 			// resolve_target: Some(self.main_texture()),
// 			ops: Operations {
// 				// load: LoadOp::Load,
// 				// load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
// 				load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
// 				store: true,
// 			},
// 		})],
// 		depth_stencil_attachment: None,
// 	});
// 	drop(pass);
// 	queue.submit(std::iter::once(encoder.finish()));
// 	// log!("howdy");
// 	// Ok(())
// }
