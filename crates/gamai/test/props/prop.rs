use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let my_tree = || {
		tree! {
			<adds_my_thing>
				<adds_my_thing/>
				<adds_my_thing/>
				<adds_my_thing>
						<group/>
				</adds_my_thing>
			</adds_my_thing>
		}
	};

	let mut app = App::new();
	app.add_plugins(TreePlugin::new(my_tree));
	// let entity = app
	// 	.world
	// 	.spawn(TreeBundle::new(tree, ActionResult::Running))
	// 	.id();
	let entity = app.world.spawn_empty().id();

	expect(Prop::<ActionResult, _>::get(my_tree, &app.world, entity))
		.to_be_none()?;
	app.update();
	expect(Prop::get(my_tree, &app.world, entity))
		.to_be(Some(&ActionResult::Success))?;


	let val = PropTree::<ActionResult>::new(my_tree, &app.world, entity);
	expect(val.to_string().as_str()).to_be(OUT)?;

	Ok(())
}

const OUT: &str = r#"Success
  Success
  Success
  Success
    None"#;

#[action]
fn adds_my_thing<Node: AiNode>(
	mut commands: Commands,
	query: Query<Entity, Without<Prop<ActionResult, Node>>>,
) {
	for entity in query.iter() {
		commands
			.entity(entity)
			.insert(Prop::<_, Node>::new(ActionResult::Success));
	}
}
