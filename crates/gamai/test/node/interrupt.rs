use crate::tests::utils::expect_tree;
use bevy_app::App;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	app.add_plugins(ActionPlugin::<BuiltinNode, _>::default());

	let target = app.world.spawn_empty().id();

	let action_graph = ActionTree::new(vec![Box::new(EmptyAction)])
		.with_leaf(vec![Box::new(EmptyAction)])
		.with_child(
			ActionTree::new(vec![Box::new(EmptyAction)])
				.with_leaf(vec![Box::new(EmptyAction)]),
		)
		.into_action_graph();

	let entity_graph = action_graph.spawn(&mut app.world, target);

	for entity in entity_graph.node_weights() {
		app.world.entity_mut(*entity).insert(Running);
	}

	expect_tree(
		&mut app,
		&entity_graph,
		Tree::new(Some(&Running))
			.with_leaf(Some(&Running))
			.with_child(Tree::new(Some(&Running)).with_leaf(Some(&Running))),
	)?;

	let entity = &entity_graph.0.clone().into_tree().children[1].value;
	app.world.entity_mut(*entity).insert(Interrupt);

	app.update();

	expect_tree(
		&mut app,
		&entity_graph,
		Tree::new(Some(&Running))
			.with_leaf(Some(&Running))
			.with_child(Tree::new(None).with_leaf(None)),
	)?;

	Ok(())
}
