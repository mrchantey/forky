use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;


#[action]
fn my_system<N: AiNode>() {}

#[tree_builder]
pub fn OtherTree(
	foo: u32,
	#[default = 8] bar: u8,
	#[default] bazz: Option<&'static str>,
) -> impl TreeElement {
	tree! {
		<my_system props=(foo,bar,bazz)/>
	}
}

#[tree_builder]
pub fn MyTree(#[default = 3] foo: u32) -> impl TreeElement {
	tree! {
		<OtherTree foo=foo/>
	}
}


#[sweet_test]
pub fn tree_props() -> Result<()> {
	let mut world = World::new();
	let entity = world.spawn(TreeBundle::inactive(MyTree)).id();
	expect(world.inspect_entity(entity).len()).to_be(3)?;
	let tree = PropTree::<u32>::new(MyTree, &world, entity);
	expect(tree.children.len()).to_be(0)?;
	expect(tree.value).as_some()?.to_be(&3)?;
	Ok(())
}
