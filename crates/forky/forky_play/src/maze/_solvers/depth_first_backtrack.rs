use crate::maze::*;
use forky_core::{graph::*, *};
use std::collections::HashSet;

pub trait DepthFirstBacktrace: Maze {
	fn depth_first_backtrack<T>(&mut self, on_next: T)
	where
		T: Fn(&MazeShadow),
	{
		let maze = self.shadow();

		maze.paths.clear_links();
		let mut visited: HashSet<usize> = HashSet::new();
		let mut stack: Vec<usize> = Vec::new();
		visited.insert(maze.head);
		stack.push(maze.head);

		while stack.len() > 0 {
			let i_cur = stack._pop();
			let unvisited = maze.nodes.next_unvisited(i_cur, &visited);

			if let Some(unvisited) = unvisited {
				let next = unvisited.clone();
				maze.paths.link(i_cur, next);
				visited.insert(next);
				stack.push(i_cur);
				stack.push(next);
			}
			on_next(&maze);
		}
	}
}
