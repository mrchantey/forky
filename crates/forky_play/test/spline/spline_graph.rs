use bevy::prelude::*;
use forky_play::spline::*;
use forky_play::*;
use sweet::*;
sweet! {
	test "spline node" {

		let spline1 = Spline::Linear(LinearSpline{
			p0: Vec3::UP,
			p1: Vec3::ZERO,
		});
		let spline2 = Spline::Linear(LinearSpline{
			p0: Vec3::ZERO,
			p1: Vec3::DOWN,
		});
		let mut node1 = SplineNode::new(spline1);
		let node2 = SplineNode::new(spline2);
		node1.add_next(&node2);

		let prev = node1.get_current_node(-0.01);
		if prev != None {
			panic!("expected none, got {:?}", prev.unwrap());
		}

		let spline = node1.get_current_node(0.5).unwrap().spline;
		if let Spline::Linear(spline) = spline {
			expect(spline.p0.y).to_be(1.)?;
		}else{
			panic!("spline is not linear");
		}
		let spline = node1.get_current_node(1.5).unwrap().spline;
		if let Spline::Linear(spline) = spline {
			expect(spline.p0.y).to_be(0.)?;
		}else{
			panic!("spline is not linear");
		}
		let last = node1.get_current_node(2.1);
		if prev != None {
			panic!("expected none, got {:?}", prev.unwrap());
		}


	}
}
