use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{
	Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ViewTarget};
use std::sync::Mutex;

use super::BlitImageHandle;




pub struct BlitNode {
	image_handle: BlitImageHandle,
}


impl BlitNode {
	pub const IN_VIEW: &'static str = "view";

	pub fn new(world: &mut World) -> Self {
		Self {
			image_handle: BlitImageHandle::default(),
		}
	}
}

impl Node for BlitNode {
	fn input(&self) -> Vec<SlotInfo> {
		vec![SlotInfo::new(BlitNode::IN_VIEW, SlotType::Entity)]
	}

	fn update(&mut self, world: &mut World) {
		self.image_handle =
			world.get_resource::<BlitImageHandle>().unwrap().clone();
	}

	fn run(
		&self,
		_graph: &mut RenderGraphContext,
		render_context: &mut RenderContext,
		world: &World,
	) -> Result<(), NodeRunError> {
		let images = world.get_resource::<RenderAssets<Image>>().unwrap();
		let src = images.get(&self.image_handle.src).unwrap();
		let dest = images.get(&self.image_handle.dest).unwrap();

		let size = Extent3d {
			width: self.image_handle.width,
			height: self.image_handle.height,
			..default()
		};

		let src = ImageCopyTexture {
			texture: &src.texture,
			mip_level: 0,
			origin: Origin3d::ZERO,
			aspect: TextureAspect::All,
		};
		let dest = ImageCopyTexture {
			texture: &dest.texture,
			mip_level: 0,
			origin: Origin3d::ZERO,
			aspect: TextureAspect::All,
		};

		render_context
			.command_encoder()
			.copy_texture_to_texture(src, dest, size);
		Ok(())
	}
}
