use super::{MazeGraph, Node};
use forky_core::*;
use std::collections::HashSet;







pub fn depth_first_backtrack<T>(graph: &mut MazeGraph, on_next:T) where T : 
Fn(&MazeGraph) {
	graph.clear();
	let mut visited: HashSet<usize> = HashSet::new();
	let mut stack: Vec<usize> = Vec::new();
	visited.insert(graph.head);
	stack.push(graph.head);

	while stack.len() > 0 {
		let i_cur = stack._pop();
		let current: &Node = &graph.nodes._get(i_cur);
		// node
		// let unvisited = current.neighbors.iter().filter(|n| !visited.contains(n)).next();
		let unvisited = graph.next_unvisited(i_cur, &visited);
		
		if let Some(unvisited) = unvisited{
			let next = unvisited.clone();
			graph.link(i_cur, next);
			visited.insert(next);
			stack.push(i_cur);
			stack.push(next);
		}
		on_next(graph);
	}
}
