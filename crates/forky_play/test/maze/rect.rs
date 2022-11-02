use forky_core::*;
use forky_play::*;
use sweet::*;

sweet! {
	let width = 5;
	let height = 3;
	test "indices"{
		let str = maze::rect::format_indices(width,height);
		// expect(&str[..]).to_be("┌┬┬┬┬┐\n├┼┼┼┼┤\n├┼┼┼┼┤\n└┴┴┴┴┘\n")?;
		log!(str);
	}

	test "grid"{
		let grid = maze::rect::draw(width,height);
		let str = maze::rect::format(grid,width,height);
		expect(&str[..]).to_be("┌┬┬┬┬┐\n├┼┼┼┼┤\n├┼┼┼┼┤\n└┴┴┴┴┘\n")?;
		// log!(str);
	}
	
	test "maze" {
		let mut graph = maze::rect::init(width,height);

		graph.link(0,1);
		graph.link(3,4);
		graph.link(11,12);
		// graph.link(1989,2898);
		
		let grid = maze::rect::draw_maze(graph,width,height);
		let str = maze::rect::format(grid,width,height);
		log!(str);
	}
}
