use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use gamai::builtin_nodes::*;
use gamai::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let tree = || {
		tree! {
			<adds_my_thing>
				<adds_my_thing/>
				<adds_my_thing/>
				<adds_my_thing>
						<empty_node/>
				</adds_my_thing>
			</adds_my_thing>
		}
	};

	let mut app = App::new();
	app.add_plugins(AiPlugin::new(tree));
	// let entity = app
	// 	.world
	// 	.spawn(PropBundle::new(tree, Lifecycle::Running))
	// 	.id();
	let entity = app.world.spawn_empty().id();

	expect(Prop::<Lifecycle, _>::get(tree, &app.world, entity)).to_be_none()?;
	app.update();
	expect(Prop::get(tree, &app.world, entity))
		.to_be(Some(&Lifecycle::Running))?;


	let val = tree.get_recursive::<Lifecycle>(&app.world, entity);
	expect(val.to_string().as_str()).to_be(OUT)?;

	Ok(())
}

const OUT: &str = r#"Running
  Running
  Running
  Running
    None"#;

#[action]
fn adds_my_thing<Node: AiNode>(
	mut commands: Commands,
	query: Query<Entity, Without<Prop<Lifecycle, Node>>>,
) {
	for entity in query.iter() {
		commands
			.entity(entity)
			.insert(Prop::<_, Node>::new(Lifecycle::default()));
	}
}
