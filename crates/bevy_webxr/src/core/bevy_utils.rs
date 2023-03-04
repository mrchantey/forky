use bevy::{prelude::*, render::{render_graph::*, render_resource::Texture, RenderApp}, core_pipeline::core_3d::MainPass3dNode};

// fn setup(app: &mut App, texture:wgpu::Texture) {
	
	// let render_app = app.sub_app_mut(RenderApp);
	// let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

	// let texture = Texture::from(texture);

	// MainPass3dNode::
	// let mut main_pass_node = MainPass3dNode::new(PassDescriptor {


	// });

	// // Add the texture to the render graph as a color attachment
	// render_graph.add_system_node(
	// 	"render_texture",
	// 	RenderResourcesNode::<Texture>::new(true),
	// );
	// render_graph
	// 	.add_node_edge("render_texture", bevy::render::node::MAIN_PASS)
	// 	.unwrap();
	// render_graph
	// 	.add_slot_edge(
	// 		bevy::render::node::MAIN_PASS,
	// 		bevy::render::pass::AddColorAttachment::COLOR_ATTACHMENT,
	// 		"render_texture",
	// 		bevy::render::pass::RenderResourcesNode::TEXTURE,
	// 	)
	// 	.unwrap();
// }
