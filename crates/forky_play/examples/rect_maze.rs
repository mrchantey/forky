use forky_core::*;
use forky_play::*;


fn main() -> ! {
	let mut count = 0;
	loop {
		count = count + 1;
		let width = 140;
		let height = 34;
		let mut graph = maze::rect::new(width, height);

		terminal::clear();
		maze::generator::depth_first_backtrack(&mut graph, |g| {
			// time::sleep_ms(16);
			terminal::reset_cursor();
			let grid = maze::rect::draw_maze(g, width, height);
			let str = maze::rect::format(&grid, width, height);
			log!(str);
			println!("making maze number {}", count);
		});

		let grid = maze::rect::draw_maze(&graph, width, height);
		let str = maze::rect::format(&grid, width, height);
		file::write(format!("./output/maze_{}.txt", count), str.as_str()).unwrap();
	}
}
