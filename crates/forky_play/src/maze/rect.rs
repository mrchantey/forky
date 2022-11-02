use forky_core::{log, StringX};

use super::{char_shape, u8_shape, MazeGraph, Node};
use std::collections::HashSet;


pub fn init(width: usize, height: usize) -> MazeGraph {
	let num_nodes = width * height;
	let mut nodes = vec![Node::new(); num_nodes];
	let head = nodes.first();

	for row in 0..height {
		for col in 0..width {
			let i = col + width * row;
			//top
			if col > 0 {
				nodes[i].neighbors.insert(i - 1);
			}
			//bottom
			if col < width - 1 {
				nodes[i].neighbors.insert(i + 1);
			}
			//left
			if row > 0 {
				nodes[i].neighbors.insert(i - width);
			}
			//right
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

pub fn draw_maze(graph: MazeGraph, width: usize, height: usize) -> Vec<u8> {
	let mut vec = draw(width, height);


	for col in 0..width {}
	for row in 0..height {
		for col in 0..width {
			if row == 0 && col < width - 1 {
				let a = col + row * width;
				let b = col + row * width + 1;
				let i = a + 1;
				if graph.is_linked(a, b) {
					vec[i] = u8_shape::HORIZONTAL;
				} else {
					vec[i] = u8_shape::TOP_TEE;
				}
			}

			// if graph.no
		}
	}
	// let s = String::from_utf8(buf).unwrap();
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


pub fn format(grid: Vec<u8>, width: usize, height: usize) -> String {
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
