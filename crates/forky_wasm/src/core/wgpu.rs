use crate::{core::*, *};
use anyhow::Result;
use bevy::prelude::*;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;




pub fn set_framebuffer(app: MutexGuard<App>, frame: &XrFrame) {
	let gl = get_webgl_context().unwrap(); //slow i think
	let gl_layer = frame.session().render_state().base_layer().unwrap();
	let framebuffer = gl_layer.framebuffer();

	let width = 100;
	let height = 100;
	let device = app.world.resource::<RenderDevice>().wgpu_device();



	let opaque_texture = device.get_texture(&wgpu::TextureDescriptor {
		size: wgpu::Extent3d {
			width: width as u32,
			height: height as u32,
			depth_or_array_layers: 1,
		},
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::Rgba8Unorm,
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT
			| wgpu::TextureUsages::TEXTURE_BINDING
			| wgpu::TextureUsages::COPY_DST,
		label: Some("opaque texture"),
	});

	// let texture = unsafe {
	// 		device.inner.create_texture_from_hal::<wgpu_hal::gles::Api>(
	// 				wgpu_hal::gles::Texture {
	// 						inner: wgpu_hal::gles::TextureInner::RawFramebuffer {
	// 								inner: framebuffer.clone(),
	// 						},
	// 						mip_level_count: 1,
	// 						array_layer_count: 1,
	// 						format: wgpu::TextureFormat::Rgba8Unorm,
	// 						format_desc: wgpu_hal::gles::TextureFormatDesc {
	// 								internal: glow::RGBA,
	// 								external: glow::RGBA,
	// 								data_type: glow::UNSIGNED_BYTE,
	// 						},
	// 						copy_size: wgpu_hal::CopyExtent {
	// 								width: base_layer.framebuffer_width(),
	// 								height: base_layer.framebuffer_height(),
	// 								depth: 1,
	// 						},
	// 				},
	// 				&wgpu::TextureDescriptor {
	// 						label: Some("framebuffer (color)"),
	// 						size: wgpu::Extent3d {
	// 								width: base_layer.framebuffer_width(),
	// 								height: base_layer.framebuffer_height(),
	// 								depth_or_array_layers: 1,
	// 						},
	// 						mip_level_count: 1,
	// 						sample_count: 1,
	// 						dimension: wgpu::TextureDimension::D2,
	// 						format: wgpu::TextureFormat::Rgba8Unorm,
	// 						usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
	// 				},
	// 		)
	// };

	// let depth = unsafe {
	// 		device.inner.create_texture_from_hal::<wgpu_hal::gles::Api>(
	// 				wgpu_hal::gles::Texture {
	// 						inner: wgpu_hal::gles::TextureInner::RawFramebuffer { inner: framebuffer },
	// 						mip_level_count: 1,
	// 						array_layer_count: 1,
	// 						format: wgpu::TextureFormat::Depth32Float,
	// 						format_desc: wgpu_hal::gles::TextureFormatDesc {
	// 								internal: glow::RGBA,
	// 								external: glow::RGBA,
	// 								data_type: glow::UNSIGNED_BYTE,
	// 						},
	// 						copy_size: wgpu_hal::CopyExtent {
	// 								width: base_layer.framebuffer_width(),
	// 								height: base_layer.framebuffer_height(),
	// 								depth: 1,
	// 						},
	// 				},
	// 				&wgpu::TextureDescriptor {
	// 						label: Some("framebuffer (depth)"),
	// 						size: wgpu::Extent3d {
	// 								width: base_layer.framebuffer_width(),
	// 								height: base_layer.framebuffer_height(),
	// 								depth_or_array_layers: 1,
	// 						},
	// 						mip_level_count: 1,
	// 						sample_count: 1,
	// 						dimension: wgpu::TextureDimension::D2,
	// 						format: wgpu::TextureFormat::Depth32Float,
	// 						usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
	// 				},
	// 		)
	// };

	// let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
	// let depth_view = depth.create_view(&wgpu::TextureViewDescriptor::default());

	// let mut encoder = device
	// 		.inner
	// 		.create_command_encoder(&wgpu::CommandEncoderDescriptor {
	// 				label: Some("command encoder"),
	// 		});

	// let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
	// 		label: Some("main render pass"),
	// 		color_attachments: &[
	// 				wgpu::RenderPassColorAttachment {
	// 						view: &view,
	// 						resolve_target: None,
	// 						ops: wgpu::Operations {
	// 								load: if mode == web_sys::XrSessionMode::ImmersiveAr {
	// 										wgpu::LoadOp::Load
	// 								} else {
	// 										wgpu::LoadOp::Clear(wgpu::Color {
	// 												r: 0.1,
	// 												g: 0.2,
	// 												b: 0.3,
	// 												a: 1.0,
	// 										})
	// 								},
	// 								store: true,
	// 						},
	// 				},
	// 				/*wgpu::RenderPassColorAttachment {
	// 						view: &opaque_texture.view,
	// 						resolve_target: None,
	// 						ops: wgpu::Operations {
	// 								load: wgpu::LoadOp::Clear(wgpu::Color {
	// 										r: 0.1,
	// 										g: 0.2,
	// 										b: 0.3,
	// 										a: 1.0,
	// 								}),
	// 								store: true,
	// 						},
	// 				},*/
	// 		],
	// 		depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
	// 				view: &depth_view,
	// 				depth_ops: Some(wgpu::Operations {
	// 						load: wgpu::LoadOp::Clear(1.0),
	// 						store: true,
	// 				}),
	// 				stencil_ops: None,
	// 		}),
	// });

	// let uniform_buffer = |i| {
	// 		BindingResource::Buffer(if i == 0 {
	// 				&uniform_buffer
	// 		} else {
	// 				&right_uniform_buffer
	// 		})
	// };

	// {
	// 		let formats = &[
	// 				wgpu::TextureFormat::Rgba8Unorm,
	// 				//wgpu::TextureFormat::Rgba8Unorm,
	// 		];

	// 		let device = device.with_formats(formats, Some(wgpu::TextureFormat::Depth32Float));

	// 		let pipeline = device.get_pipeline(
	// 				"plane pipeline",
	// 				device.device.get_shader(
	// 						"shaders/compiled/plane.vert.spv",
	// 						#[cfg(feature = "standalone")]
	// 						include_bytes!("../shaders/compiled/plane.vert.spv"),
	// 						Default::default(),
	// 				),
	// 				device.device.get_shader(
	// 						"shaders/compiled/plane.frag.spv",
	// 						#[cfg(feature = "standalone")]
	// 						include_bytes!("../shaders/compiled/plane.frag.spv"),
	// 						BindGroupLayoutSettings {
	// 								//external_texture_slots: &[4],
	// 								..Default::default()
	// 						},
	// 				),
	// 				RenderPipelineDesc {
	// 						primitive: wgpu::PrimitiveState {
	// 								// as we're flipping things in the shaders.
	// 								cull_mode: Some(wgpu::Face::Front),
	// 								..Default::default()
	// 						},
	// 						depth_compare: wgpu::CompareFunction::Less,
	// 						..Default::default()
	// 				},
	// 				BASIC_FORMAT,
	// 		);

	// 		render_pass.set_pipeline(&pipeline.pipeline);
}
