use forky_play::*;
use sweet::*;

sweet! {
	
	it "works" {
		app::init()
			.add_plugin(utility::basic::BasicPlugin)
			.add_system(utility::create_exit_after_system(3))
			.run();
	}
}


