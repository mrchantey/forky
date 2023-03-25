use anyhow::*;
use bevy::prelude::*;
use forky_play::spline::*;
use forky_play::*;
use sweet::*;
sweet! {
	test "spline graph" {

		let mut graph = SplineGraph::new();

		let spline1 = Spline::Linear(LinearSpline{
			p0: Vec3::UP,
			p1: Vec3::ZERO,
		});
		let spline2 = Spline::Linear(LinearSpline{
			p0: Vec3::ZERO,
			p1: Vec3::DOWN,
		});
		let node1 = graph.create_node();
		let node2 = graph.create_node();
		let node3 = graph.create_node();
		expect(*node1).to_be(0)?;
		expect(*node2).to_be(1)?;
		expect(*node1).not().to_be(*node2)?;

		let edge12 = graph.create_edge(node1,node2,spline1);
		let edge23 = graph.create_edge(node2,node3,spline2);

		let prev = graph.get_current_spline(&edge12,-0.01);
		if prev != None {
			panic!("expected none, got {:?}", prev.unwrap());
		}

		let spline = graph.get_current_spline(&edge12,0.5).unwrap().spline;
		if let Spline::Linear(spline) = spline {
			expect(spline.p0.y).to_be(1.)?;
		}else{
			panic!("spline is not linear");
		}
		let spline = graph.get_current_spline(&edge12,1.5).unwrap().spline;
		if let Spline::Linear(spline) = spline {
			expect(spline.p0.y).to_be(0.)?;
		}else{
			panic!("spline is not linear");
		}
		let last = graph.get_current_spline(&edge12,2.1);
		if prev != None {
			panic!("expected none, got {:?}", prev.unwrap());
		}


	}
}
