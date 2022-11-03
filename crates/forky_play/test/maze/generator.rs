use forky_core::*;
use forky_play::*;
use sweet::*;

sweet! {
	let width = 50;
	let height = 20;

	it "works" {
		let mut graph = maze::rect::new(width,height);
		maze::generator::depth_first_backtrack(&mut graph,|g|{
			let grid = maze::rect::draw_maze(g,width,height);
			let str = maze::rect::format(&grid,width,height);
			// log!(str);
		});


		let grid = maze::rect::draw_maze(&graph,width,height);
		let str = maze::rect::format_indices(width,height);
		// log!(str);
		let str = maze::rect::format(&grid,width,height);
		// log!(str);
		// dir!(graph);
		// expect(true).to_be(false)?;

	}
}
