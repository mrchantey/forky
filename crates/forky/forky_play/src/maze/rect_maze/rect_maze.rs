use crate::maze::*;
use bevy::prelude::*;
use forky_core::graph::*;
use forky_core::*;

#[derive(Debug, Component)]
pub struct RectMaze {
	pub num_cols: usize,
	pub num_rows: usize,
	pub head: usize,
	pub tail: usize,
	pub nodes: NodeGraph,
	pub paths: NodeGraph,
}

impl DepthFirstBacktrace for RectMaze {}

impl Default for RectMaze {
	fn default() -> Self { RectMaze::new(10, 10) }
}

impl Maze for RectMaze {
	// fn nodes(&self) -> &NodeGraph { &self.nodes }
	// fn paths_mut(&mut self) -> &mut NodeGraph { &mut self.paths }
	// fn head(&self) -> usize { self.head }

	fn shadow<'a>(&'a mut self) -> MazeShadow<'a> {
		MazeShadow {
			head: self.head,
			nodes: &self.nodes,
			paths: &mut self.paths,
		}
	}
}

impl RectMaze {
	pub fn new(num_cols: usize, num_rows: usize) -> RectMaze {
		// let num_nodes = num_cols * num_rows;
		let mut nodes = NodeGraph::from_len(num_cols * num_rows);
		let paths = NodeGraph::from_len(num_cols * num_rows);

		for row in 0..num_rows {
			for col in 0..num_cols {
				let i = col + num_cols * row;
				//left
				if col > 0 {
					nodes[i].links.insert(i - 1);
				}
				//right
				if col < num_cols - 1 {
					nodes[i].links.insert(i + 1);
				}
				//top
				if row > 0 {
					nodes[i].links.insert(i - num_cols);
				}
				//bottom
				if row < num_rows - 1 {
					nodes[i].links.insert(i + num_cols);
				}
			}
		}
		RectMaze {
			num_cols,
			num_rows,
			head: 0,
			tail: nodes.len() - 1,
			nodes,
			paths,
		}
	}

	pub fn draw_maze(&self) -> Vec<u8> {
		let mut vec = self.draw_grid();
		let e_width = self.num_cols + 1;
		let _e_height = self.num_rows + 1;


		for row in 0..self.num_rows {
			for col in 0..self.num_cols {
				let c1 = col + row * self.num_cols;
				let c2 = col + row * self.num_cols + 1;
				let c3 = c2 + self.num_cols;
				let c4 = c1 + self.num_cols;
				let e1 = col + row * e_width + 1;
				let e3 = e1 + e_width;
				let e2 = e3 + 1;
				let e4 = e3 - 1;
				let c1_c2_linked = self.paths.is_linked(c1, c2);
				let c2_c3_linked = self.paths.is_linked(c2, c3);
				let c3_c4_linked = self.paths.is_linked(c3, c4);
				let c4_c1_linked = self.paths.is_linked(c4, c1);
				//handle row edges
				if row == 0 && col < self.num_cols - 1 {
					if c1_c2_linked {
						vec[e1] = u8_shape::HORIZONTAL;
					} else {
						vec[e1] = u8_shape::TOP_TEE;
					}
				}
				if row == self.num_rows - 1 && col < self.num_cols - 1 {
					if c1_c2_linked {
						vec[e3] = u8_shape::HORIZONTAL;
					} else {
						vec[e3] = u8_shape::BOTTOM_TEE;
					}
				}
				//handle column edges
				if col == 0 && row < self.num_rows - 1 {
					if c4_c1_linked {
						vec[e4] = u8_shape::VERTICAL;
					} else {
						vec[e4] = u8_shape::LEFT_TEE;
					}
				}
				if col == self.num_cols - 2 && row < self.num_rows - 1 {
					if c2_c3_linked {
						vec[e2] = u8_shape::VERTICAL;
					} else {
						vec[e2] = u8_shape::RIGHT_TEE;
					}
				}
				//handle centers
				if row < self.num_rows - 1 && col < self.num_cols - 1 {
					if c1_c2_linked
						&& c2_c3_linked && c3_c4_linked
						&& c4_c1_linked
					{
						vec[e3] = u8_shape::NONE;
					} else if c1_c2_linked && c2_c3_linked && c3_c4_linked {
						vec[e3] = u8_shape::HORIZONTAL_LEFT;
					} else if c2_c3_linked && c3_c4_linked && c4_c1_linked {
						vec[e3] = u8_shape::VERTICAL_TOP;
					} else if c3_c4_linked && c4_c1_linked && c1_c2_linked {
						vec[e3] = u8_shape::HORIZONTAL_RIGHT;
					} else if c4_c1_linked && c1_c2_linked && c2_c3_linked {
						vec[e3] = u8_shape::VERTICAL_BOTTOM;
					} else if c1_c2_linked && c3_c4_linked {
						vec[e3] = u8_shape::HORIZONTAL;
					} else if c4_c1_linked && c2_c3_linked {
						vec[e3] = u8_shape::VERTICAL;
					} else if c1_c2_linked && c2_c3_linked {
						vec[e3] = u8_shape::TOP_RIGHT;
					} else if c2_c3_linked && c3_c4_linked {
						vec[e3] = u8_shape::BOTTOM_RIGHT;
					} else if c3_c4_linked && c4_c1_linked {
						vec[e3] = u8_shape::BOTTOM_LEFT;
					} else if c4_c1_linked && c1_c2_linked {
						vec[e3] = u8_shape::TOP_LEFT;
					} else if c1_c2_linked {
						vec[e3] = u8_shape::TOP_TEE;
					} else if c2_c3_linked {
						vec[e3] = u8_shape::RIGHT_TEE;
					} else if c3_c4_linked {
						vec[e3] = u8_shape::BOTTOM_TEE;
					} else if c4_c1_linked {
						vec[e3] = u8_shape::LEFT_TEE;
					} else {
						vec[e3] = u8_shape::CROSS;
					}
				}
			}
		}

		//handle tail - only works for bottom edge
		let tail_edges = self.node_to_edges(self.tail);
		vec[tail_edges.2] = self.open_left(vec[tail_edges.2]);
		vec[tail_edges.3] = self.open_right(vec[tail_edges.3]);

		vec
	}
	/*
	e1,e2
	e3,e4
	*/
	fn node_to_edges(&self, node: usize) -> (usize, usize, usize, usize) {
		let col = node % self.num_cols;
		let row = node / self.num_cols; //floor
		let e_num_cols = self.num_cols + 1;
		let e1 = col + row * e_num_cols;
		let e2 = e1 + 1;
		let e3 = e1 + e_num_cols;
		let e4 = e3 + 1;
		(e1, e2, e3, e4)
	}

	pub fn open_left(&self, shape: u8) -> u8 {
		match shape {
			u8_shape::BOTTOM_RIGHT => u8_shape::VERTICAL_TOP,
			u8_shape::BOTTOM_LEFT => u8_shape::VERTICAL_TOP,
			u8_shape::BOTTOM_TEE => u8_shape::BOTTOM_RIGHT,
			u8_shape::HORIZONTAL => u8_shape::HORIZONTAL_LEFT,
			other => other,
		}
	}
	pub fn open_right(&self, shape: u8) -> u8 {
		match shape {
			u8_shape::BOTTOM_LEFT => u8_shape::VERTICAL_TOP,
			u8_shape::BOTTOM_RIGHT => u8_shape::VERTICAL_TOP,
			u8_shape::BOTTOM_TEE => u8_shape::BOTTOM_LEFT,
			u8_shape::HORIZONTAL => u8_shape::HORIZONTAL_RIGHT,

			other => other,
		}
	}

	pub fn draw_grid(&self) -> Vec<u8> {
		let mut buf: Vec<u8> = Vec::new();
		for row in 0..self.num_rows + 1 {
			for col in 0..self.num_cols + 1 {
				if row == 0 && col == 0 {
					buf.push(u8_shape::TOP_LEFT);
				} else if row == 0 && col == self.num_cols {
					buf.push(u8_shape::TOP_RIGHT);
				} else if row == self.num_rows && col == self.num_cols {
					buf.push(u8_shape::BOTTOM_RIGHT);
				} else if row == self.num_rows && col == 0 {
					buf.push(u8_shape::BOTTOM_LEFT);
				} else if row == 0 {
					buf.push(u8_shape::TOP_TEE);
				} else if row == self.num_rows {
					buf.push(u8_shape::BOTTOM_TEE);
				} else if col == 0 {
					buf.push(u8_shape::LEFT_TEE);
				} else if col == self.num_cols {
					buf.push(u8_shape::RIGHT_TEE);
				} else {
					buf.push(u8_shape::CROSS);
				}
			}
		}
		buf
	}

	pub fn format_grid(&self) -> String { self.format_vec(self.draw_grid()) }
	pub fn format(&self) -> String { self.format_vec(self.draw_maze()) }

	/*
	|_|_|
	|_|_|
	c1 e1 c2
	e4    e2
	c4 e3 c3
	*/
	fn format_vec(&self, grid: Vec<u8>) -> String {
		let mut str = String::new();

		for row in 0..self.num_rows + 1 {
			for col in 0..self.num_cols + 1 {
				let i = col + row * (self.num_cols + 1);
				str.push(char_shape::from_u8(grid[i]));
			}
			str.push('\n');
		}
		str
	}

	pub fn format_indices(&self) -> String {
		let mut str = String::new();

		for row in 0..self.num_rows {
			for col in 0..self.num_cols {
				let i = col + row * self.num_cols;
				str.push_string(&format!("{}\t", i));
			}
			str.push('\n');
		}
		str
	}

	pub fn link_randomly(&mut self) { self.paths.link_randomly(&self.nodes); }
}
