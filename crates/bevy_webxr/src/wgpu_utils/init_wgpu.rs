use wgpu::{InstanceDescriptor, Queue, Device};
use anyhow::Result;

use crate::*;

struct RawWindowHandle;

unsafe impl raw_window_handle::HasRawWindowHandle for RawWindowHandle {
	fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
		let mut web = raw_window_handle::WebWindowHandle::empty();
		web.id = 666;

		raw_window_handle::RawWindowHandle::Web(web)
	}
}
unsafe impl raw_window_handle::HasRawDisplayHandle for RawWindowHandle {
	fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
		let mut web = raw_window_handle::WebDisplayHandle::empty();
		raw_window_handle::RawDisplayHandle::Web(web)
	}
}


pub async fn init_wgpu()->Result<(Device,Queue)> {
	let instance = wgpu::Instance::new(InstanceDescriptor::default());
	let surface = unsafe { instance.create_surface(&RawWindowHandle).unwrap() };
	let adapter = instance
		.request_adapter(&wgpu::RequestAdapterOptions {
			power_preference: wgpu::PowerPreference::HighPerformance,
			force_fallback_adapter: false,
			compatible_surface: Some(&surface),
		})
		.await
		.expect("No suitable GPU adapters found on the system!");

	let adapter_info = adapter.get_info();
	// log!(
	// 	"Using {} with the {:?} backend",
	// 	adapter_info.name,
	// 	adapter_info.backend
	// );

	let result = adapter
		.request_device(
			&wgpu::DeviceDescriptor {
				label: Some("device"),
				features: Default::default(),
				limits: wgpu::Limits {
					// max_texture_dimension_1d: framebuffer_width
					// 	.max(framebuffer_height)
					// 	.max(2048),
					// max_texture_dimension_2d: framebuffer_width
					// 	.max(framebuffer_height)
					// 	.max(2048),
					..wgpu::Limits::downlevel_webgl2_defaults()
				},
			},
			None,
		)
		.await
		.expect("Unable to find a suitable GPU adapter!");
	Ok(result)
}
