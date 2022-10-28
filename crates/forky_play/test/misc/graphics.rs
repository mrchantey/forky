use forky_play::*;
use forky_test::*;

describe!("graphics", |s| {
	s.skip().it("works", || {
		ForkyApp::init()
		.add_plugin(BasicPlugin)
			.add_system(create_exit_after_system(3))
			.run();
		Ok(())
	});
});