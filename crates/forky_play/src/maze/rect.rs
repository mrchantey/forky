use forky_core::{log, StringX};

use super::{char_shape, u8_shape, MazeGraph, Node};
use std::collections::HashSet;


pub fn new(width: usize, height: usize) -> MazeGraph {
	let num_nodes = width * height;
	let mut nodes = vec![Node::new(); num_nodes];
	let head = nodes.first();

	for row in 0..height {
		for col in 0..width {
			let i = col + width * row;
			//left
			if col > 0 {
				nodes[i].neighbors.insert(i - 1);
			}
			//right
			if col < width - 1 {
				nodes[i].neighbors.insert(i + 1);
			}
			//top
			if row > 0 {
				nodes[i].neighbors.insert(i - width);
			}
			//bottom
			if row < height - 1 {
				nodes[i].neighbors.insert(i + width);
			}
		}
	}
	let graph = MazeGraph {
		nodes,
		head: 0,
		tail: num_nodes - 1,
	};
	graph
	// head
}
/*
|_|_|
|_|_|
c1 e1 c2
e4    e2
c4 e3 c3

*/
pub fn draw_maze(graph: &MazeGraph, width: usize, height: usize) -> Vec<u8> {
	let mut vec = draw(width, height);
	let e_width = width + 1;
	let e_height = height + 1;


	for row in 0..height {
		for col in 0..width {
			let c1 = col + row * width;
			let c2 = col + row * width + 1;
			let c3 = c2 + width;
			let c4 = c1 + width;
			let e1 = col + row * e_width + 1;
			let e3 = e1 + e_width;
			let e2 = e3 + 1;
			let e4 = e3 - 1;
			let c1_c2_linked = graph.is_linked(c1, c2);
			let c2_c3_linked = graph.is_linked(c2, c3);
			let c3_c4_linked = graph.is_linked(c3, c4);
			let c4_c1_linked = graph.is_linked(c4, c1);
			//handle row edges
			if row == 0 && col < width - 1 {
				if c1_c2_linked {
					vec[e1] = u8_shape::HORIZONTAL;
				} else {
					vec[e1] = u8_shape::TOP_TEE;
				}
			} if row == height - 1 && col < width - 1 {
				if c1_c2_linked {
					vec[e3] = u8_shape::HORIZONTAL;
				} else {
					vec[e3] = u8_shape::BOTTOM_TEE;
				}
			}
			//handle column edges
			if col == 0 && row < height - 1 {
				if c4_c1_linked {
					vec[e4] = u8_shape::VERTICAL;
				} else {
					vec[e4] = u8_shape::LEFT_TEE;
				}
			} 
			if col == width - 2 && row < height - 1 {
				if c2_c3_linked {
					vec[e2] = u8_shape::VERTICAL;
				} else {
					vec[e2] = u8_shape::RIGHT_TEE;
				}
			}
			//handle centers
			if row < height - 1 && col < width - 1 {
				if c1_c2_linked && c2_c3_linked && c3_c4_linked && c4_c1_linked {
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
	vec
}


pub fn draw(width: usize, height: usize) -> Vec<u8> {
	let mut buf: Vec<u8> = Vec::new();
	for row in 0..height + 1 {
		for col in 0..width + 1 {
			if row == 0 && col == 0 {
				buf.push(u8_shape::TOP_LEFT);
			} else if row == 0 && col == width {
				buf.push(u8_shape::TOP_RIGHT);
			} else if row == height && col == width {
				buf.push(u8_shape::BOTTOM_RIGHT);
			} else if row == height && col == 0 {
				buf.push(u8_shape::BOTTOM_LEFT);
			} else if row == 0 {
				buf.push(u8_shape::TOP_TEE);
			} else if row == height {
				buf.push(u8_shape::BOTTOM_TEE);
			} else if col == 0 {
				buf.push(u8_shape::LEFT_TEE);
			} else if col == width {
				buf.push(u8_shape::RIGHT_TEE);
			} else {
				buf.push(u8_shape::CROSS);
			}
		}
	}
	buf
}


pub fn format(grid: &Vec<u8>, width: usize, height: usize) -> String {
	let mut str = String::new();

	for row in 0..height + 1 {
		for col in 0..width + 1 {
			let i = col + row * (width + 1);
			str.push(char_shape::from_u8(grid[i]));
		}
		str.push('\n');
	}
	str
}
pub fn format_indices(width: usize, height: usize) -> String {
	let mut str = String::new();

	for row in 0..height {
		for col in 0..width {
			let i = col + row * width;
			str.push_string(&format!("{}\t", i));
		}
		str.push('\n');
	}
	str
}
