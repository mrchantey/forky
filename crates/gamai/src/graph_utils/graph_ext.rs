use crate::prelude::*;
use bevy_utils::HashMap;
use bevy_utils::HashSet;
use extend::ext;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use petgraph::Direction;


#[ext]
pub impl<N, E> DiGraph<N, E> {
	fn root(&self) -> Option<&N> { self.node_weight(NodeIndex::new(0)) }
	fn node(&self, index: usize) -> Option<&N> {
		self.node_weight(NodeIndex::new(index))
	}
}

#[ext(name=DiGraphExtUnitEdge)]
pub impl<N> DiGraph<N, ()> {
	fn from_tree(root: Tree<N>) -> Self {
		let mut this = Self::new();
		this.from_tree_recursive(root.into());
		this
	}

	fn from_tree_recursive(&mut self, tree: Tree<N>) -> NodeIndex {
		let Tree::<N> { value, children } = tree;
		let node = self.add_node(value);

		// why do we need to reverse this?
		for child in children.into_iter().rev() {
			let index = self.from_tree_recursive(child);
			self.add_edge(node, index, ());
		}

		node
	}

	/// Take nodes from a graph, preserving node indices.
	fn take_nodes(mut self) -> HashMap<NodeIndex, N> {
		let mut nodes = HashMap::default();
		for index in self.node_indices().rev() {
			nodes.insert(index, self.remove_node(index).unwrap());
		}
		nodes
	}


	/// Discards unused edges.
	fn into_tree(self) -> Tree<N> {
		let index_tree = self
			.index_tree_recursive(NodeIndex::new(0), &mut HashSet::default());
		let mut node_map = self.take_nodes();
		Self::index_tree_to_tree(index_tree, &mut node_map)
	}

	fn index_tree_to_tree(
		index_tree: Tree<NodeIndex>,
		node_map: &mut HashMap<NodeIndex, N>,
	) -> Tree<N> {
		let Tree::<NodeIndex> { value, children } = index_tree;
		let children = children
			.into_iter()
			.map(|child| Self::index_tree_to_tree(child, node_map))
			.collect::<Vec<_>>();

		Tree {
			value: node_map.remove(&value).unwrap(),
			children,
		}
	}

	/// Note: This will empty the graph.
	fn index_tree_recursive(
		&self,
		parent: NodeIndex,
		visited: &mut HashSet<NodeIndex>,
	) -> Tree<NodeIndex> {
		visited.insert(parent);

		Tree {
			value: parent,
			children: self
				.neighbors_directed(parent, Direction::Outgoing)
				.filter(|index| !visited.contains(index))
				.collect::<Vec<_>>()
				.into_iter()
				.map(|index| self.index_tree_recursive(index, visited))
				.collect(),
		}
	}
}

// deprecated in favour of into_tree

// #[ext(name=DiGraphExtPartialEq)]
// pub impl<N, E> DiGraph<N, E>
// where
// 	N: PartialEq,
// {
// 	/// Untested on graphs that have had edges removed and re-added.
// 	fn equals_tree(&self, tree: &Tree<N>) -> bool {
// 		self.equals_tree_recursive(tree, NodeIndex::default())
// 	}
// 	fn equals_tree_recursive(&self, tree: &Tree<N>, start: NodeIndex) -> bool {
// 		if self.node_weight(start) != Some(&tree.value) {
// 			false
// 		} else {
// 			self.neighbors(start).enumerate().all(|(i, child)| {
// 				self.equals_tree_recursive(&tree.children[i], child)
// 			})
// 		}
// 	}
// }
