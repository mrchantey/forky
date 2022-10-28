use bevy::prelude::*;
use forky_test::*;


struct Score(u32);

#[derive(Component)]
struct Pos {
	x: f32,
	y: f32,
}

fn increment_score(mut score: ResMut<Score>) { score.0 += 1; }

describe!("bevy", |s| {
	s.it("runs", || {
		let mut app = App::new();
		app.insert_resource(Score(0)).add_system(increment_score);

		app.update();
		expect(app.world.resource::<Score>().0).to_be(1)?;
		Ok(())
	});


	s.it("fooo", || {
		let mut app = App::new();
		let id = app.world.spawn().insert(Pos { x: 0., y: 1. }).id();
		app.update();
		let result = app.world.get::<Pos>(id).unwrap();
		expect(result.x).to_be(0.)?;
		expect(result.y).to_be(1.)?;
		Ok(())
	})
});
