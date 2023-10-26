use bevy_app::App;
use gamai::common_actions::*;
use gamai::common_selectors::*;
use gamai::*;
use sweet::*;

#[tree_builder]
pub fn MyTree() -> impl AiNode {
	tree! {
		<highest_score>
			<empty_node before_parent=score_always_fail/>
			<empty_node before_parent=score_always_pass/>
		</highest_score>
	}
}

#[sweet_test]
pub fn it_works() -> Result<()> {
	let mut app = App::new();

	app.add_plugins(TreePlugin::new(MyTree));

	let entity = app
		.world
		.spawn((
			TreeBundle::recursive(MyTree, Score::Fail),
			TreeBundle::root(MyTree, Running),
		))
		.id();
	app.update();

	let tree = PropTree::<Running>::new(MyTree, &app.world, entity);

	expect(tree.children[0].value).to_be_none()?;
	expect(tree.children[1].value).to_be_some()?;

	Ok(())
}
