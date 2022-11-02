use forky_core::*;
use rand::Rng;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug)]
pub struct MazeGraph {
	pub nodes: Vec<Node>,
	pub head: usize,
	pub tail: usize,
}

impl MazeGraph {
	pub fn clear(&mut self) {
		for node in self.nodes.iter_mut() {
			node.links.clear();
		}
	}
	pub fn is_linked(&self, a: usize, b: usize) -> bool {
		if a >= self.nodes.len() || b >= self.nodes.len() {
			return false;
		}
		self.nodes[a].links.contains(&b)
	}


	pub fn link_randomly(&mut self) {
		let iter = self.nodes.iter().enumerate();
		let mut to_link: Vec<(usize, usize)> = Vec::new();
		for (i, node) in self.nodes.iter().enumerate() {
			for neighbor in node.neighbors.iter() {
				let num = rand::thread_rng().gen_range(0..100);
				if num < 20 {
					to_link.push((i, neighbor.clone()));
					// self.link(i, neighbor.clone());
				}
			}
		}
		to_link.iter().for_each(|(a, b)| self.link(*a, *b));
		// self.nodes[a].links.insert(b);
		// self.nodes[b].links.insert(a);
	}


	pub fn link(&mut self, a: usize, b: usize) {
		self.nodes[a].links.insert(b);
		self.nodes[b].links.insert(a);
	}
	pub fn unlink(&mut self, a: usize, b: usize) {
		self.nodes[a].links.remove(&b);
		self.nodes[b].links.remove(&a);
	}

	pub fn next_unvisited(&self, i: usize, visited: &HashSet<usize>) -> Option<&usize> {
		self.nodes
			._get(i)
			.neighbors
			.iter()
			.filter(|n| !visited.contains(n))
			.next()
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
