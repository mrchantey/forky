use bevy::prelude::*;
use bevy::render::render_graph::{
	Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ViewTarget};

use super::CustomClearColor;




pub struct ClearNode {
	query: QueryState<&'static ViewTarget, With<ExtractedView>>,
	clear_color: CustomClearColor,
}


impl ClearNode {
	pub const IN_VIEW: &'static str = "view";

	pub fn new(world: &mut World) -> Self {
		Self {
			query: QueryState::new(world),
			clear_color: CustomClearColor::default(),
		}
	}
}

impl Node for ClearNode {
	fn input(&self) -> Vec<SlotInfo> {
		vec![SlotInfo::new(ClearNode::IN_VIEW, SlotType::Entity)]
	}

	fn update(&mut self, world: &mut World) {
		self.query.update_archetypes(world);
		self.clear_color = *world.get_resource::<CustomClearColor>().unwrap();
	}

	fn run(
		&self,
		_graph: &mut RenderGraphContext,
		render_context: &mut RenderContext,
		world: &World,
	) -> Result<(), NodeRunError> {
		// let view_entity = graph.get_input_entity(Self::IN_VIEW)?;
		// let (target) = match self.query.get_manual(world, view_entity) {
		// 	Ok(query) => query,
		// 	Err(_) => return Ok(()),
		// };
		for target in self.query.iter_manual(world) {
			let pass_descriptor = RenderPassDescriptor {
				label: Some("Clear Pass"),
				color_attachments: &[Some(target.get_color_attachment(
					Operations {
						// load: LoadOp::Load,
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: self.clear_color.r,
							g: self.clear_color.g,
							b: self.clear_color.b,
							a: self.clear_color.a,
						}),
						store: true,
					},
				))],
				depth_stencil_attachment: None,
			};

			render_context
				.command_encoder()
				.begin_render_pass(&pass_descriptor);
		}
		Ok(())
	}
}
