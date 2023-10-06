use bevy_app::App;
use gamai::*;
use sweet::*;


#[tree_builder]
pub fn MyTree() -> impl AiTree {
	tree! {
		<first_valid_edge>
			<empty_node edge=edge_always_fail/>
			<empty_node edge=edge_always_pass/>
		</first_valid_edge>
	}
}

#[sweet_test]
pub fn it_works() -> Result<()> {
	let mut app = App::new();

	app.add_plugins(MyTree.plugin());

	let entity = app.world.spawn(MyTree.bundle()).id();
	app.update();

	expect(MyTree.child(0).node_state(&app.world, entity))
		.to_be_none()?;
	expect(MyTree.child(1).node_state(&app.world, entity))
		.to_be(Some(NodeState::Running))?;

	Ok(())
}

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
