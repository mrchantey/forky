use forky_core::graph::NodeGraph;
pub use forky_core::graph::*;

pub struct MazeShadow<'a> {
	pub head: usize,
	pub nodes: &'a NodeGraph,
	pub paths: &'a mut NodeGraph,
}

pub trait Maze {
	fn shadow<'a>(&'a mut self) -> MazeShadow<'a>;
	// fn head(&self) -> usize;
	// fn nodes(&self) -> &NodeGraph;
	// fn paths_mut(&mut self) -> &mut NodeGraph;
}
