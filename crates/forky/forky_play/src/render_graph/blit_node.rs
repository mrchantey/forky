use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{
	Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;



use super::BlitImageHandle;




pub struct BlitNode {
	image_handle: BlitImageHandle,
}


impl BlitNode {
	pub const IN_VIEW: &'static str = "view";

	pub fn new(_world: &mut World) -> Self {
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

		// let src = ImageCopyTexture {
		// 	texture: &src.texture,
		// 	mip_level: 0,
		// 	origin: Origin3d::ZERO,
		// 	aspect: TextureAspect::All,
		// };
		// let dest = ImageCopyTexture {
		// 	texture: &dest.texture,
		// 	mip_level: 0,
		// 	origin: Origin3d::ZERO,
		// 	aspect: TextureAspect::All,
		// };

		render_context.command_encoder().copy_texture_to_texture(
			src.texture.as_image_copy(),
			dest.texture.as_image_copy(),
			size,
		);
		// .copy_texture_to_texture(src, dest, size);
		Ok(())
	}
}



// let pass_descriptor = RenderPassDescriptor {
// 	label: Some("Blit Pass"),
// 	color_attachments: &[
// 		Some(RenderPassColorAttachment {
// 			view: &src.texture_view,
// 			// view: &target_view,
// 			resolve_target: None,
// 			ops: wgpu::Operations {
// 				load: wgpu::LoadOp::Load,
// 				store: true,
// 			},
// 		}),
// 		Some(RenderPassColorAttachment {
// 			view: dest_view,
// 			resolve_target: None,
// 			ops: wgpu::Operations {
// 				load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
// 				store: true,
// 			},
// 		}),
// 	],
// 	depth_stencil_attachment: None,
// };
// render_context
// 	.command_encoder()
// 	.begin_render_pass(&pass_descriptor);
