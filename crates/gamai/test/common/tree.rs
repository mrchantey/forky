use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;

type F<const I: usize> = ChildEdgeState<EdgePhantom<my_node, I>>;
type A<const I: usize> = ChildNodeState<EdgePhantom<my_node, I>>;


// #[node(32)]
#[node]
fn my_node<N: AiNode>() {}
// fn my_node(){
// 	// my_nodePlugin::new(node, edges)
// 	first_valid_edge
// }

sweet! {
	it "works" {
		let edge0 = EdgeBuilder::new(edge_always_fail, noop_node);
		let edge1 = EdgeBuilder::new(edge_always_pass, noop_node);
		// // let edge0 = (edge_always_fail, print_on_run);
		// // let edge1 = (edge_always_pass, print_on_run);
		let mut world = World::new();
		let entity = world.spawn(my_node_bundle::default()).id();
		expect(&world).not().to_have_component::<A<0>>(entity)?;
		expect(&world).to_have_component::<F<0>>(entity)?;
		let mut schedule = Schedule::default();
		my_node_plugin::new(first_valid_edge, (edge0, edge1)).build(&mut schedule);
		schedule.run(&mut world);
		expect(&world)
			.component::<F<0>>(entity)?
			.map(|a| a.state)
			.to_be(EdgeState::Fail)?;
		expect(&world)
			.component::<F<1>>(entity)?
			.map(|a| a.state)
			.to_be(EdgeState::Pass)?;
		expect(&world).not().to_have_component::<A<0>>(entity)?;
		expect(&world).to_have_component::<A<1>>(entity)?;
	}
}
