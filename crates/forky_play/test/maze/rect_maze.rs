#![allow(unused_mut)]
use forky_core::*;
use forky_play::{
	maze::{rect_maze::RectMaze, *},
	*,
};
use sweet::*;

sweet! {

	before {
		let width = 5;
		let height = 3;
		let mut maze = RectMaze::new(width,height);
	}

	test "indices"{
		let str = maze.format_indices();
		expect(&str[..]).to_be("0\t1\t2\t3\t4\t\n5\t6\t7\t8\t9\t\n10\t11\t12\t13\t14\t\n")?;
	}

	test "grid"{
		let str = maze.format_grid();
		expect(&str[..]).to_be("┌┬┬┬┬┐\n├┼┼┼┼┤\n├┼┼┼┼┤\n└┴┴┴┴┘\n")?;
	}

	test "maze" {
		maze.paths.link(0,1);
		maze.paths.link(0,5);
		maze.paths.link(3,4);
		maze.paths.link(7,8);
		maze.paths.link(9,14);
		maze.paths.link(11,12);
		let str = maze.format();
		expect(&str[..]).to_be("┌─┬┬─┐\n│┌┼┴┬┤\n├┼┴┬┤│\n└┴─┴┘╵\n")?;
		// log!(str);
	}

	test "tail"{
		expect(&maze.format()[..]).to_be("┌┬┬┬┬┐\n├┼┼┼┼┤\n├┼┼┼┼┤\n└┴┴┴┘╵\n")?;
		maze.tail = 13;
		expect(&maze.format()[..]).to_be("┌┬┬┬┬┐\n├┼┼┼┼┤\n├┼┼┼┼┤\n└┴┴┘└┘\n")?;
		maze.paths.link(13,14);
		expect(&maze.format()[..]).to_be("┌┬┬┬┬┐\n├┼┼┼┼┤\n├┼┼┼┴┤\n└┴┴┘╶┘\n")?;
	}

	test skip "random" {
		let width = 10;
		let height = 5;
		maze.link_randomly();
		let str = maze.format();
		log!(str);
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
			let mut maze = RectMaze::new(width,height);
			for (a,b) in arr.iter(){
				maze.paths.link(*a as usize,*b as usize);
			}
			// let str = maze.format();
			// log!(str);
		}
	}
}
