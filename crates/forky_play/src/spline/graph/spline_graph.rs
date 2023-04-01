use super::*;
use crate::{spline::Spline};



use petgraph::{
	graphmap::{UnGraphMap},
};

#[derive(Debug, Clone)]
pub struct SplineGraph {
	pub next_node_id: u64,
	pub graph: UnGraphMap<SplineNode, SplineEdge>,
}

impl std::ops::Deref for SplineGraph {
	type Target = UnGraphMap<SplineNode, SplineEdge>;
	fn deref(&self) -> &Self::Target { &self.graph }
}

impl std::ops::DerefMut for SplineGraph {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.graph }
}


impl SplineGraph {
	pub fn new() -> Self {
		Self {
			next_node_id: 0,
			graph: UnGraphMap::new(),
		}
	}

	pub fn create_node(&mut self) -> SplineNode {
		let node = SplineNode(self.next_node_id);
		self.add_node(node);
		self.next_node_id += 1;
		node
	}

	pub fn create_edge(
		&mut self,
		node1: SplineNode,
		node2: SplineNode,
		spline: Spline,
	) -> SplineEdge {
		let edge = SplineEdge::new(node1, node2, spline);
		self.add_edge(node1, node2, edge.clone());
		edge
	}

	pub fn get_current_spline(
		&self,
		edge: &SplineEdge,
		t: f32,
	) -> Option<SplineEdge> {
		let edge = match self.edge_weight(edge.a, edge.b) {
			Some(value) => value,
			None => return None,
		};

		if t >= 0.0 && t <= 1.0 {
			return Some(edge.clone());
		} else if t < 0.0 {
			for next in self.edges(edge.a) {
				if next.2 != edge {
					return self.get_current_spline(next.2, t + 1.);
				}
			}
		} else if t > 1.0 {
			for next in self.edges(edge.b) {
				if next.2 != edge {
					return self.get_current_spline(next.2, t - 1.);
				}
			}
		}
		return None;
	}
}
