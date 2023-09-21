// use bevy_ecs::prelude::*;
// use gamai::*;
// use sweet::*;

// type MyTree = gamai::tree!(
// 	<first_valid_edge>
// 		<print_on_run/>
// 		<print_on_run/>
// 	</first_valid_edge>
// );
// type Child0 = <MyTree as NamedChildren2>::Child0;
// type Child1 = <MyTree as NamedChildren2>::Child1;

// sweet! {

// 	test "behavior tree" {
// 		let mut world = World::new();
// 		let mut schedule = Schedule::default();
// 		MyTree::add_systems(&mut schedule);
// 		let entity = world.spawn(MyTree::default()).id();
// 		expect(&world)
// 			.to_have_component::<ChildEdgeState<Child0>>(entity)?;
// 		expect(&world).not()
// 			.to_have_component::<ChildNodeState<Child0>>(entity)?;
// 		schedule.run(&mut world);
// 		// schedule.run(&mut world);
// 		expect(&world)
// 			.component::<ChildEdgeState<Child0>>(entity)?
// 			.map(|a| a.state)
// 			.to_be(EdgeState::Fail)?;
// 		expect(&world)
// 			.component::<ChildEdgeState<Child1>>(entity)?
// 			.map(|a| a.state)
// 			.to_be(EdgeState::Pass)?;
// 		expect(&world).not().to_have_component::<ChildNodeState<Child0>>(entity)?;
// 		expect(&world).to_have_component::<ChildNodeState<Child1>>(entity)?;
// 	}
// }
