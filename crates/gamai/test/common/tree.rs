use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;

type MyGraph = gamai::graph!(
	<first_valid_edge edge=edge_always_pass>
		<noop_node edge=edge_always_fail/>
		<noop_node edge=edge_always_pass/>
		// <print_on_run edge=edge_always_pass/>
	</first_valid_edge>
);
type Child0 = <MyGraph as NamedChildren2>::Child0;
type Child1 = <MyGraph as NamedChildren2>::Child1;

sweet! {
	it "works" {
		let mut world = World::new();
		let mut schedule = Schedule::default();
		MyGraph::build(&mut schedule);
		let entity = world.spawn(MyGraph::default()).id();
		expect(&world)
			.to_have_component::<ChildEdgeState<Child0>>(entity)?;
		expect(&world).not()
			.to_have_component::<ChildNodeState<Child0>>(entity)?;
		schedule.run(&mut world);
		// schedule.run(&mut world);
		expect(&world)
			.component::<ChildEdgeState<Child0>>(entity)?
			.map(|a| a.state)
			.to_be(EdgeState::Fail)?;
		expect(&world)
			.component::<ChildEdgeState<Child1>>(entity)?
			.map(|a| a.state)
			.to_be(EdgeState::Pass)?;
		expect(&world).not().to_have_component::<ChildNodeState<Child0>>(entity)?;
		expect(&world).to_have_component::<ChildNodeState<Child1>>(entity)?;
	}
}
