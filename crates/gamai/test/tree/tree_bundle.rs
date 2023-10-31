use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;

#[derive(Debug)]
struct Foo;
#[derive(Debug, PartialEq)]
struct Bar(pub bool);

#[action(
	props = (Foo,Bar(true))
)]
pub fn my_action() {}

#[sweet_test]
pub fn works() -> Result<()> {
	let my_tree = || {
		tree! {
			<group>
				<group/>
				<my_action/>
			</group>
		}
	};

	let mut world = World::new();
	let entity = world.spawn(TreeBundle::new(my_tree)).id();

	let tree = PropTree::<Foo>::new(my_tree, &world, entity);
	expect(tree.value).to_be_none()?;
	expect(tree.children[0].value).to_be_none()?;
	expect(tree.children[1].value).to_be_some()?;

	let tree = PropTree::<Bar>::new(my_tree, &world, entity);
	expect(tree.value).to_be_none()?;
	expect(tree.children[0].value).to_be_none()?;
	expect(tree.children[1].value).to_be(Some(&Bar(true)))?;

	Ok(())
}
