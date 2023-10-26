// use forky_fs::*;
use forky_play::maze::{rect_maze::RectMaze, *};


fn main() -> ! {
	let mut count = 0;
	loop {
		count = count + 1;
		let width = 140;
		let height = 34;
		let mut maze = RectMaze::new(width, height);

		// terminal::clear();
		maze.depth_first_backtrack(|_g| {
			// time::sleep_ms(16);
			// terminal::reset_cursor();
			// let grid = maze::_rect::draw_maze(g, width, height);
			// let str = maze::_rect::format(&grid, width, height);
			// log!(str);
			// println!("making maze number {}", count);
		});

		// let str = maze.format();
		// file::write(format!("./output/maze_{}.txt", count), str.as_str())
		// 	.unwrap();
	}
}
