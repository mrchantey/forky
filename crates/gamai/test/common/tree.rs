use bevy_app::prelude::*;
// use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;

fn my_node() -> impl AiNode {
	tree! {
		<sequence>
		<node_always_succeed/>
		</sequence>
	}
}
#[sweet_test]
pub fn it_works() -> Result<()> {
	let mut app = App::new();
	app.add_plugins(AiPlugin::new(my_node));
	// let entity = app.world.spawn(AiBundle::running(my_node)).id();

	// let child = my_node.child0();
	// expect(my_node.node_state(&app.world, entity))
	// 	.to_be(Some(NodeState::Running))?;

	// app.update();

	// expect(my_node.node_state(&app.world, entity))
	// 	.to_be(Some(NodeState::Success))?;

	//TODO how do we test this?
	Ok(())
}


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
