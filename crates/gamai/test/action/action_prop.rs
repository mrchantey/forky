use bevy_app::App;
use gamai::*;
use sweet::*;

#[derive(Debug)]
struct Foo;

#[action(props=Foo)]
fn foo() {}

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();

	let tree = || {
		tree! {
			<foo apply_deferred>
				<foo/>
			</foo>
		}
	};

	let entity = app.world.spawn(TreeBundle::new(tree)).id();
	let tree = PropTree::<Foo>::new(tree, &app.world, entity);
	expect(tree.value).to_be_some()?;
	expect(tree.children[0].value).to_be_some()?;
	Ok(())
}
