use forky_play::*;
use forky_test::*;

describe!("graphics", |s| {
	s.it("works", || {
		app::init()
			.add_plugin(utility::basic::BasicPlugin)
			.add_system(utility::create_exit_after_system(1))
			.run();
		Ok(())
	});
});
