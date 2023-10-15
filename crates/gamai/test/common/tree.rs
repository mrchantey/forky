use bevy_app::App;
use gamai::*;
use sweet::*;


#[tree_builder]
pub fn MyTree() -> impl AiNode {
	tree! {
		<sequence>
			<node_always_succeed/>
		</sequence>
	}
}

#[sweet_test]
pub fn it_works() -> Result<()> {
	let mut app = App::new();

	app.add_plugins(AiPlugin::new(MyTree));

	let entity = app
		.world
		.spawn(PropBundle::root(MyTree, NodeState::Running))
		.id();

	expect(Prop::<NodeState, _>::get(MyTree, &app.world, entity))
		.to_be(Some(&NodeState::Running))?;

	// app.update();
	// app.update();

	// expect(Prop::<NodeState, _>::get(MyTree, &app.world, entity))
	// 	.to_be(Some(&NodeState::Success))?;

	Ok(())
}
