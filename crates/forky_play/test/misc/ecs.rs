use bevy::prelude::*;
use sweet::*;

#[derive(Resource)]
struct Score(u32);

#[derive(Component)]
struct Pos {
	x: f32,
	y: f32,
}

fn increment_score(mut score: ResMut<Score>) { score.0 += 1; }

sweet! {
	test "resources" {
		let mut app = App::new();
		app.insert_resource(Score(0)).add_systems(Update,increment_score);

		app.update();
		expect(app.world.resource::<Score>().0).to_be(1)?;
	}


	test "components" {
		let mut app = App::new();
		let id = app.world.spawn_empty().insert(Pos { x: 0., y: 1. }).id();
		app.update();
		let result = app.world.get::<Pos>(id).unwrap();
		expect(result.x).to_be(0.)?;
		expect(result.y).to_be(1.)?;
	}
}
