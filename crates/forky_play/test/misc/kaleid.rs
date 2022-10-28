use forky_test::*;
use forky_play::*;

describe!("kaleid", |s| {

	s.it("works", || {
		ForkyApp::init()
		.add_plugin(Kaleid)
		// .add_plugin(BasicPlugin)
		.add_startup_system(BasicPlugin::spawn_camera)
		.add_startup_system(||{})

			// .add_system(create_exit_after_system(3))
			.run();
		
		Ok(())
	});
});