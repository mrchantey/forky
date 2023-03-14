use crate::*;
use anyhow::Result;
// use bevy::prelude::*;
// use bevy::render::render_phase::AddRenderCommand;
// use wgpu::*;
use bevy::render::renderer::RenderDevice;
// use std::cell::RefCell;
// use std::rc::Rc;
// use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::MutexGuard;
// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::*;
// use wgpu::*;


// const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

pub fn create_framebuffer_texture(
	device: &wgpu::Device,
	gl_layer: &XrWebGlLayer,
) -> wgpu::Texture {
	unsafe {
		device.create_texture_from_hal::<wgpu_hal::gles::Api>(
			wgpu_hal::gles::Texture {
				inner: wgpu_hal::gles::TextureInner::ExternalFramebuffer {
					// inner: framebuffer,
					inner: gl_layer.framebuffer_unwrapped(),
					// inner: framebuffer.as_ref().unwrap().clone(),
				},
				mip_level_count: 1,
				array_layer_count: 1,
				format: wgpu::TextureFormat::Rgba8Unorm, //TODO check this is ok, different from bevy default
				format_desc: wgpu_hal::gles::TextureFormatDesc {
					internal: glow::RGBA,
					external: glow::RGBA,
					data_type: glow::UNSIGNED_BYTE,
				},
				copy_size: wgpu_hal::CopyExtent {
					width: gl_layer.framebuffer_width(),
					height: gl_layer.framebuffer_height(),
					depth: 1,
				},
				drop_guard: None,
				is_cubemap: false,
			},
			&wgpu::TextureDescriptor {
				label: Some("framebuffer (color)"),
				size: wgpu::Extent3d {
					width: gl_layer.framebuffer_width(),
					height: gl_layer.framebuffer_height(),
					depth_or_array_layers: 1,
				},
				mip_level_count: 1,
				sample_count: 1,
				dimension: wgpu::TextureDimension::D2,
				format: FORMAT,
				view_formats: &[FORMAT],
				usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // | wgpu::TextureUsages::COPY_SRC,
				// | wgpu::TextureUsages::COPY_DST,
				                                               // wgpu::TextureUsages::TEXTURE_BINDING
																											},
																										)
																									}
																								}