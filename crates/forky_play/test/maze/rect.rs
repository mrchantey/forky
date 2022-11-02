use forky_core::*;
use forky_play::*;
use sweet::*;

sweet! {
	let width = 5;
	let height = 3;
	test "indices"{
		let str = maze::rect::format_indices(width,height);
		expect(&str[..]).to_be("0\t1\t2\t3\t4\t\n5\t6\t7\t8\t9\t\n10\t11\t12\t13\t14\t\n")?;
		// log!(str);
	}

	test "grid"{
		let grid = maze::rect::draw(width,height);
		let str = maze::rect::format(&grid,width,height);
		expect(&str[..]).to_be("┌┬┬┬┬┐\n├┼┼┼┼┤\n├┼┼┼┼┤\n└┴┴┴┴┘\n")?;
		// log!(str);
	}

	test "maze" {
		let mut graph = maze::rect::new(width,height);

		graph.link(0,1);
		graph.link(0,5);
		graph.link(3,4);
		graph.link(7,8);
		graph.link(9,14);
		graph.link(11,12);
		// graph.link(1989,2898);

		let grid = maze::rect::draw_maze(&graph,width,height);
		let str = maze::rect::format(&grid,width,height);
		// log!(maze::rect::format_indices(width,height));
		// log!(str);
	}
	test "big" {
		let width = 10;
		let height = 5;
		let mut graph = maze::rect::new(width,height);
		graph.link_randomly();
		// dir!(graph);
		let grid = maze::rect::draw_maze(&graph,width,height);
		let str = maze::rect::format(&grid,width,height);
		// log!(str);
	}
	test "options" {
		let width = 2;
		let height = 2;

		let v12 = (0,1);
		let v23 = (1,3);
		let v34 = (3,2);
		let v41 = (2,0);

		let arrs = vec![
			vec![v12,v23,v34,v41],
			vec![v12,v23,v34],
			vec![v12,v23],
			vec![v12],
			vec![v23,v34,v41],
			vec![v23,v34],
			vec![v23],
			vec![v34,v41,v12],
			vec![v34,v41],
			vec![v34],
			vec![v41,v12,v23],
			vec![v41,v12],
			vec![v41],
			vec![v12,v34],
			vec![v23,v41],
			vec![],
		];

		for arr in arrs.iter(){
			let mut graph = maze::rect::new(width,height);
			for (a,b) in arr.iter(){
				graph.link(*a as usize,*b as usize);
			}
		let grid = maze::rect::draw_maze(&graph,width,height);
		// let str = maze::rect::format(&grid,width,height);
		// log!(str);

		}


		// dir!(graph);
		// log!(maze::rect::format_indices(width,height));
	}
}
