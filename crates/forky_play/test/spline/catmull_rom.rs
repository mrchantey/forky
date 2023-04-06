use bevy::prelude::*;
use forky_play::spline::graph::*;
use forky_play::*;
use sweet::*;

sweet! {
	it "works" {

		let mut graph = SplineGraph::new();

		let node1 = graph.create_node(Vec3::UP);
		let node2 = graph.create_node(Vec3::ZERO);
		let node3 = graph.create_node(Vec3::RIGHT);

		graph.create_edge_linear(node1,node2);
		graph.create_edge_linear(node2,node3);


		expect((node1,node2)).not().to_be((node2,node1))?;


		let solved = graph.solve_catmull_rom();
		expect(solved.len()).to_be(4)?;

		// println!("{:?}",graph);
		// 	println!("\n");
		// for result in solved.iter() {
		// 	println!("{:?}",result);
		// }

	}
}
