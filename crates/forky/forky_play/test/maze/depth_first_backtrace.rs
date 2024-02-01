use forky_play::maze::rect_maze::RectMaze;
use forky_play::maze::*;
use sweet::*;

sweet! {

	it "works" {
		let width = 50;
		let height = 20;
		let mut graph = RectMaze::new(width,height);
		expect(graph.nodes.len()).to_be(width*height)?;
		expect(graph.paths.len()).to_be(width*height)?;

		graph.depth_first_backtrack(|_s|{
			// let a = graph.head;
			// 	// let grid = maze::_rect::draw_maze(g,width,height);
			// 	// let str = maze::_rect::format(&grid,width,height);
			// 	// log!(str);
		});

		let _indices = graph.format_indices();
		let _maze = graph.format();
		// log!(indices);
		// log!(maze);
		// dir!(graph);
	}
}
