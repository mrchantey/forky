use std::collections::HashSet;

#[derive(Debug)]
pub struct MazeGraph {
	pub nodes: Vec<Node>,
	pub head: usize,
	pub tail: usize,
}

impl MazeGraph {
	pub fn is_linked(&self, a: usize, b: usize) -> bool { self.nodes[a].links.contains(&b) }
	pub fn link(&mut self, a: usize, b: usize) {
		self.nodes[a].links.insert(b);
		self.nodes[b].links.insert(a);
	}
	pub fn unlink(&mut self, a: usize, b: usize) {
		self.nodes[a].links.remove(&b);
		self.nodes[b].links.remove(&a);
	}
}

#[derive(Clone, Debug)]
pub struct Node {
	pub neighbors: HashSet<usize>,
	pub links: HashSet<usize>,
}

impl Node {
	pub fn new() -> Node {
		Node {
			neighbors: HashSet::new(),
			links: HashSet::new(),
		}
	}
}
