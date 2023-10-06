// use bevy_app::App;
// use gamai::*;
// use sweet::*;

// fn my_node() -> impl AiNode {
// 	tree! {<node_always_succeed/>}
// }


// #[sweet_test]
// pub fn it_works() -> Result<()> {
// 	let mut app = App::new();
// 	app.add_plugins(AiPlugin::new(my_node));
// 	let entity = app.world.spawn(AiBundle::running(my_node)).id();

// 	expect(my_node.node_state(&app.world, entity))
// 		.to_be(Some(NodeState::Running))?;

// 	app.update();

// 	expect(my_node.node_state(&app.world, entity))
// 		.to_be(Some(NodeState::Success))?;

// 	Ok(())
// }
