use super::{rect, MazeGraph};

pub fn new(width: usize) -> MazeGraph { rect::new(width, width) }
pub fn draw_maze(graph: &MazeGraph, width: usize) -> Vec<u8> { rect::draw_maze(graph, width, width) }
pub fn draw(width: usize) -> Vec<u8> { rect::draw(width, width) }
pub fn format(grid: &Vec<u8>, width: usize) -> String { rect::format(grid, width, width) }
