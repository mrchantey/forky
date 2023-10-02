// use bevy_ecs::prelude::*;
// use gamai::*;
// use sweet::*;

// type MyTree = gamai::tree!(
// 	<first_valid_edge edge=edge_always_pass>
// 		<empty_node edge=edge_always_fail/>
// 		<empty_node edge=edge_always_pass/>
// 		// <print_on_run edge=edge_always_pass/>
// 	</first_valid_edge>
// );
// type Child0 = <MyTree as NamedChildren2>::Child0;
// type Child1 = <MyTree as NamedChildren2>::Child1;

// sweet! {

// 	test "utility tree" {
// 		let mut world = World::new();
// 		let mut schedule = Schedule::default();
// 		MyTree::add_systems(&mut schedule);
// 		let entity = world.spawn(MyTree::bundle()).id();
// 		expect(&world)
// 			.to_have_component::<DerefEdgeState<MyTree>>(entity)?;
// 		expect(&world)
// 			.to_have_component::<DerefNodeState<MyTree>>(entity)?;
// 		expect(&world)
// 			.to_have_component::<DerefEdgeState<Child0>>(entity)?;
// 		expect(&world).not()
// 			.to_have_component::<DerefNodeState<Child0>>(entity)?;
// 		schedule.run(&mut world);
// 		// schedule.run(&mut world);
// 		expect(&world)
// 			.component::<DerefEdgeState<Child0>>(entity)?
// 			.map(|a| a.state)
// 			.to_be(EdgeState::Fail)?;
// 		expect(&world)
// 			.component::<DerefEdgeState<Child1>>(entity)?
// 			.map(|a| a.state)
// 			.to_be(EdgeState::Pass)?;
// 		expect(&world).not().to_have_component::<DerefNodeState<Child0>>(entity)?;
// 		expect(&world).to_have_component::<DerefNodeState<Child1>>(entity)?;
// 	}
// }
