use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{
	Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ViewTarget};
use std::sync::Mutex;

use super::*;




pub struct BlitNode;
//  {
// image_handle: BlitImageHandle,
// }


// impl BlitNode {
// pub const IN_VIEW: &'static str = "view";

// pub fn new(world: &mut World) -> Self {
// 	Self {
// 		image_handle: BlitImageHandle::default(),
// 	}
// }
// }

impl Node for BlitNode {
	// fn input(&self) -> Vec<SlotInfo> {
	// 	vec![SlotInfo::new(BlitNode::IN_VIEW, SlotType::Entity)]
	// }

	// fn update(&mut self, world: &mut World) {
	// 	self.image_handle =
	// 		world.get_resource::<BlitImageHandle>().unwrap().clone();
	// }

	fn run(
		&self,
		_graph: &mut RenderGraphContext,
		render_context: &mut RenderContext,
		world: &World,
	) -> Result<(), NodeRunError> {
		let target = world.get_resource::<BlitTarget>();

		if let None = target {
			return Ok(());
		}
		let target = target.unwrap();

		let images = world.get_resource::<RenderAssets<Image>>().unwrap();
		let src = images.get(&target.src).unwrap();
		// let dest = images.get(&self.image_handle.dest).unwrap();
		// let src:wgpu::Texture = src.texture.into();
		// src.value.
		let size = Extent3d {
			width: target.width,
			height: target.height,
			..default()
		};

		let dest_view = &target.dest.create_view(&TextureViewDescriptor {
			label: Some("Blit Target"),
			format: Some(wgpu::TextureFormat::Rgba8Unorm),
			dimension: Some(wgpu::TextureViewDimension::D2),
			aspect: TextureAspect::All,
			..default()
		});

		// let src: &wgpu::Texture = &src.texture;

		// unsafe{
		// 	let src: &wgpu_hal::gles::Texture = &src.into();
		// }
		// src.
		// let src = ImageCopyTexture {
		// 	texture: &src,
		// 	// texture: &src.texture,
		// 	mip_level: 0,
		// 	origin: Origin3d::ZERO,
		// 	aspect: TextureAspect::All,
		// };
		// let dest = ImageCopyTexture {
		// 	texture: &target.dest,
		// 	mip_level: 0,
		// 	origin: Origin3d::ZERO,
		// 	aspect: TextureAspect::All,
		// };

		render_context
			.command_encoder()
			.copy_texture_to_texture(src.texture.as_image_copy(), target.dest.as_image_copy(), size);
		Ok(())
	}
}
