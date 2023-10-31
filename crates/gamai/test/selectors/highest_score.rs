use bevy_app::App;
use gamai::common_actions::*;
use gamai::common_selectors::*;
use gamai::*;
use sweet::*;

// #[tree_builder]
// pub fn MyTree() -> impl TreeElement {
// 	tree! {
// 		<highest_score apply_deferred>
// 		<group actions=(node_always_succeed.apply_deferred(true),score_always_fail)/>
// 		<group actions=(node_always_succeed.apply_deferred(true),score_always_pass)/>
// 		</highest_score>
// 	}
// }

// #[sweet_test]
// pub fn it_works() -> Result<()> {
// 	let mut app = App::new();

// 	app.add_plugins(TreePlugin::new(MyTree));

// 	let entity = app.world.spawn(TreeBundle::new(MyTree)).id();

// 	app.update();

// 	let tree = PropTree::<ActionResult>::new(MyTree, &app.world, entity);
// 	expect(tree.value).to_be_none()?;
// 	expect(tree.children[0].value).to_be_none()?;
// 	expect(tree.children[1].value).to_be_some()?;

// 	app.update();

// 	let tree = PropTree::<ActionResult>::new(MyTree, &app.world, entity);
// 	expect(tree.value).to_be_some()?;
// 	expect(tree.children[0].value).to_be_none()?;
// 	expect(tree.children[1].value).to_be_none()?;
// 	Ok(())
// }
