use bevy_app::prelude::*;
use gamai::*;
use sweet::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum State {
	#[default]
	Awesome,
	Terrible,
}

fn tree() -> impl AiNode {
	tree! {
		<empty_node>
			<empty_node/>
			<empty_node/>
			<empty_node>
				<empty_node/>
			</empty_node>
		</empty_node>
	}
}

#[sweet_test]
pub fn root() -> Result<()> {
	let mut app = App::new();
	let entity = app
		.world
		.spawn(PropBundle::root(tree, State::default()))
		.id();
	let out = PropTree::<State>::new(tree, &app.world, entity);
	expect(out.value).to_be(Some(&State::default()))?;
	expect(out.children[0].value).to_be_none()?;
	Ok(())
}
#[sweet_test]
pub fn recursive() -> Result<()> {
	let mut app = App::new();
	let entity = app
		.world
		.spawn(PropBundle::recursive(tree, State::Terrible))
		.id();
	let out = PropTree::<State>::new(tree, &app.world, entity);
	expect(out.value).to_be(Some(&State::Terrible))?;
	expect(out.children[0].value).to_be(Some(&State::Terrible))?;
	Ok(())
}
#[sweet_test]
pub fn flatten() -> Result<()> {
	let mut app = App::new();
	let entity = app
		.world
		.spawn(PropBundle::recursive(tree, State::Terrible))
		.id();
	let out = PropTree::<State>::new(tree, &app.world, entity);
	expect(out.flatten()).to_be(vec![Some(&State::Terrible); 5])?;
	Ok(())
}
