use bevy::prelude::*;
use forky_test::*;


struct Score(u32);

fn increment_score(mut score: ResMut<Score>) { score.0 += 1; }

describe!("bevy", |s| {
	s.it("runs", || {
		let mut app = App::new();
		app.insert_resource(Score(0)).add_system(increment_score);

		app.update();
		expect(app.world.resource::<Score>().0).to_be(1)?;
		Ok(())
	});

	// s.it("is awesome",||{
	// 	expect(0).to_be(1)?;
	// 	Ok(())
	// });
});
