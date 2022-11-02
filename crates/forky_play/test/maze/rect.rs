use forky_core::*;
use forky_play::*;
use sweet::*;

sweet! {
	let width = 5;
	let height = 3;

	test "grid"{
		let grid = maze::rect::draw(width,height);
		let str = maze::rect::format(grid,width,height);
		expect(&str[..]).to_be("┌┬┬┬┬┐\n├┼┼┼┼┤\n├┼┼┼┼┤\n└┴┴┴┴┘\n")?;
		// log!(str);
	}
	
	test "maze" {
		let mut graph = maze::rect::init(width,height);
		
		graph.nodes[0].links.insert(1);
		graph.nodes[1].links.insert(0);
		
		let grid = maze::rect::draw_maze(graph,width,height);
		let str = maze::rect::format(grid,width,height);
		log!(str);
	}
}
