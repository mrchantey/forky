use bevy_app::App;
use gamai::*;
use sweet::*;


#[tree_builder]
pub fn MyTree() -> impl AiNode {
	tree! {
		<first_passing_score>
			<empty_node edge=edge_always_fail/>
			<empty_node edge=edge_always_pass/>
		</first_passing_score>
	}
}

#[sweet_test]
pub fn it_works() -> Result<()> {
	let mut app = App::new();

	app.add_plugins(AiPlugin::new(MyTree));

	let entity = app
		.world
		.spawn((
			PropBundle::recursive(MyTree, Score::Fail),
			PropBundle::root(MyTree, NodeState::Running),
		))
		.id();
	app.update();

	let tree = PropTree::<NodeState>::new(MyTree, &app.world, entity);

	expect(tree.children[0].value).to_be_none()?;
	expect(tree.children[1].value).to_be(Some(&NodeState::Running))?;

	Ok(())
}
