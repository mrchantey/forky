use bevy_app::App;
// use gamai::common_actions::*;
// use gamai::common_selectors::*;
use gamai::*;
use sweet::*;


#[action(props = 9u32)]
fn foobar() {}


// #[tree_builder]
// pub fn MyTree() -> impl TreeElement {
// 	tree! {
// 		<group>
// 		// <group actions=(node_always_succeed.apply_deferred(true),score_always_fail)/>
// 			<group actions={
// 				tree!{
// 					<foobar/>
// 				}
// 			}/>
// 			// <group actions=foobar/>
// 		</group>
// 	}
// }

#[sweet_test]
pub fn it_works() -> Result<()> {
	let mut app = App::new();

	let my_tree = || {
		tree! {<group actions=foobar/>}				//Prop<u32, ParentNode0<ParentNode0<TreePathRoot<5>>>>
		// tree! {<group actions=tree!{<foobar/>}/>} //Prop<u32, ParentNode0<ParentNode0<ParentNode0<TreePathRoot<5>>>>>
	};
	// let my_tree = || {
	// 	tree! {
	// 		<group actions=foobar>
	// 			<foobar/>
	// 			<group actions=foobar/>
	// 			<group actions=tree!{<foobar/>}/>
	// 		</group>
	// 	}
	// };

	// app.add_plugins(TreePlugin::new(MyTree));

	let entity = app.world.spawn(TreeBundle::inactive(my_tree)).id();
	println!(
		"{}",
		app.world
			.inspect_entity(entity)
			.iter()
			.map(|val| val.name())
			.collect::<Vec<_>>()
			.join("\n\n")
			.replace("gamai::prop::", "")
			.replace("gamai::node::", "")
	);
	// expect(app.world.inspect_entity(entity).len()).to_be(2)?;

	let tree = PropTree::<u32>::new(my_tree, &app.world, entity);
	println!("{tree}");

	// expect(tree.value).to_be_none()?;
	// expect(tree.children[0].value).to_be_some()?;
	// // expect(tree.children[0].value).to_be_none()?;
	// expect(tree.children[1].value).to_be_some()?;

	// app.update();

	// let tree = PropTree::<ActionResult>::new(MyTree, &app.world, entity);
	// expect(tree.value).to_be_some()?;
	// expect(tree.children[0].value).to_be_none()?;
	// expect(tree.children[1].value).to_be_none()?;
	Ok(())
}
