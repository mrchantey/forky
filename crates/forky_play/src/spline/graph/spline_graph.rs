use super::*;
use crate::spline::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::HashSet;
use petgraph::graphmap::UnGraphMap;

#[derive(Debug, Clone, Default)]
pub struct SplineGraph {
	pub next_edge_id: u64,
	pub next_node_id: u64,
	//TODO hide behind immutable getter
	pub graph: UnGraphMap<SplineNode, SplineEdge>,
	pub positions: HashMap<SplineNode, Vec3>,
}

impl std::ops::Deref for SplineGraph {
	type Target = UnGraphMap<SplineNode, SplineEdge>;
	fn deref(&self) -> &Self::Target { &self.graph }
}

impl std::ops::DerefMut for SplineGraph {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.graph }
}


impl SplineGraph {
	pub fn new() -> Self { Self::default() }

	pub fn clone_edges(&self, node: SplineNode) -> Vec<SplineEdge> {
		self.edges(node)
			.map(|(_, _, edge)| *edge)
			.collect::<Vec<_>>()
	}

	pub fn create_node(&mut self, position: Vec3) -> SplineNode {
		let node = SplineNode(self.next_node_id);
		self.add_node(node);
		self.positions.insert(node, position);
		self.next_node_id += 1;
		node
	}

	pub fn next_neighbour(
		&mut self,
		node: SplineNode,
		prev: SplineNode,
	) -> Option<SplineNode> {
		for next in self.neighbors(node) {
			if next != prev {
				return Some(next);
			}
		}
		None
	}

	pub fn create_edge_linear(
		&mut self,
		node1: SplineNode,
		node2: SplineNode,
	) -> SplineEdge {
		let spline = Spline::Linear(LinearSpline {
			p0: self.positions[&node1],
			p1: self.positions[&node2],
		});
		self.create_edge(node1, node2, spline)
	}

	pub fn create_edge(
		&mut self,
		node1: SplineNode,
		node2: SplineNode,
		spline: Spline,
	) -> SplineEdge {
		let edge = SplineEdge::new(
			SplineEdgeId(self.next_edge_id),
			node1,
			node2,
			spline,
		);
		self.add_edge(node1, node2, edge.clone());
		self.next_edge_id += 1;
		edge
	}

	pub fn get_current_edge(
		&self,
		edge: &SplineEdge,
		t: f32,
	) -> Option<(f32, SplineEdge)> {
		//checks for changed or removed
		let edge = match self.edge_weight(edge.a, edge.b) {
			Some(value) => value,
			None => return None,
		};
		if t >= 0.0 && t <= 1.0 {
			return Some((t, edge.clone()));
		} else if t < 0.0 {
			//TODO this just gets the next edge, it should be angle based or something
			for next in self.edges(edge.a) {
				if next.2 != edge {
					return self.get_current_edge(next.2, t + 1.);
				}
			}
		} else if t > 1.0 {
			for next in self.edges(edge.b) {
				if next.2 != edge {
					return self.get_current_edge(next.2, t - 1.);
				}
			}
		}
		return None;
	}


	pub fn solve_catmull_rom(
		&mut self,
	) -> HashMap<(SplineNode, SplineNode), Vec3> {
		let mut visited: HashSet<SplineNode> = HashSet::new();
		let mut solved: HashMap<(SplineNode, SplineNode), Vec3> =
			HashMap::new();
		let first = self.positions.keys().next().unwrap();
		self.solve_catmull_rom_recursive(*first, &mut visited, &mut solved);
		solved
	}

	pub fn solve_catmull_rom_recursive(
		&mut self,
		node: SplineNode,
		visited: &mut HashSet<SplineNode>,
		solved: &mut HashMap<(SplineNode, SplineNode), Vec3>,
	) {
		if visited.contains(&node) {
			return;
		}
		visited.insert(node);
		let node_pos = self.positions[&node];
		for node_next in self
			.graph
			.neighbors(node)
			// .filter(|next| !visited.contains(&next))
			.collect::<Vec<_>>()
		{
			self.solve_catmull_rom_recursive(node_next, visited, solved);
			let node_next_pos = self.positions[&node_next];
			// println!("\nvisiting {node}, next: {node_next}");
			if let Some(node_prev) = self.next_neighbour(node, node_next) {
				//TODO currently double calculating for middle nodes
				let node_prev_pos = self.positions[&node_prev];
				let (handle_prev, handle_next) = CatmullRom::solve_three(
					node_pos,
					node_prev_pos,
					node_next_pos,
				);
				// println!("linking {node},{node_prev}");
				solved.insert((node, node_prev), handle_prev);
				// println!("linking {node},{node_next}");
				solved.insert((node, node_next), handle_next);
			} else {
				// println!("linking {node},{node_next}");
				let handle_next =
					CatmullRom::solve_two(node_pos, node_next_pos);
				solved.insert((node, node_next), handle_next);
			}
		}
	}
}
