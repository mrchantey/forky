use std::borrow::Cow;

use bevy::core_pipeline::core_3d::graph;
// use bevy::core_pipeline::core_3d:;
// use bevy::core_pipeline::prelude;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::render_graph::{Node, *};


pub const END_MAIN_PASS: &'static str =
	graph::node::END_MAIN_PASS_POST_PROCESSING;
pub const FINAL_PASS: &'static str = graph::node::UPSCALING;
pub const BLIT_PASS: &'static str = "blit_pass";
pub const CLEAR_SRC_PASS: &'static str = "clear_src_pass";
pub const CLEAR_PASS: &'static str = "clear_pass";
pub const EMPTY_PASS: &'static str = "";

pub fn insert_node<T>(
	render_app: &mut App,
	node: T,
	name: &'static str,
	prev: &'static str,
	next: &'static str,
	// in_view: &'static str,
) where
	T: Node,
{
	let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

	let draw_3d_graph = render_graph.get_sub_graph_mut(graph::NAME).unwrap();
	draw_3d_graph.add_node(name, node);

	// draw_3d_graph.add_node_edge(next, name);
	draw_3d_graph.add_node_edge(prev, name);
	if next != EMPTY_PASS {
		draw_3d_graph.add_node_edge(name, next);
		draw_3d_graph.remove_node_edge(prev, next).unwrap();
	}
}

// pub fn insert_view_node<T>(
// 	render_app: &mut App,
// 	node: T,
// 	name: &'static str,
// 	in_view: &'static str,
// 	prev: &'static str,
// 	next: &'static str,
// ) where
// 	T: Node,
// {
// 	insert_node(render_app, node, name, prev, next);
// 	//what does this do?
// 	let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
// 	let draw_3d_graph = render_graph.get_sub_graph_mut(graph::NAME).unwrap();
// 	let input_node_id = draw_3d_graph.input_node().id;
// 	draw_3d_graph.add_slot_edge(
// 		input_node_id,
// 		graph::input::VIEW_ENTITY,
// 		name,
// 		in_view,
// 	);
// }
