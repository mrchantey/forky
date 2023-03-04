use std::borrow::Cow;

use bevy::core_pipeline::core_3d::graph;
// use bevy::core_pipeline::core_3d:;
// use bevy::core_pipeline::prelude;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::render_graph::{Node, *};

pub fn insert_final_node<T>(
	render_app: &mut App,
	node: T,
	name: &'static str,
	in_view: &'static str,
) where
	T: Node,
{
	let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

	let draw_3d_graph = render_graph.get_sub_graph_mut(graph::NAME).unwrap();
	draw_3d_graph.add_node(name, node);
	// graph_3d.add_node(name, EmptyNode);


	//what does this do?
	let input_node_id = draw_3d_graph.input_node().unwrap().id;
	draw_3d_graph
		.add_slot_edge(
			input_node_id,
			graph::input::VIEW_ENTITY,
			name,
			in_view,
		)
		.unwrap();


	draw_3d_graph
		.add_node_edge(graph::node::END_MAIN_PASS_POST_PROCESSING, name)
		.unwrap();
	draw_3d_graph
		.add_node_edge(name, graph::node::UPSCALING)
		.unwrap();

	draw_3d_graph
		.remove_node_edge(
			graph::node::END_MAIN_PASS_POST_PROCESSING,
			graph::node::UPSCALING,
		)
		.unwrap();
}
