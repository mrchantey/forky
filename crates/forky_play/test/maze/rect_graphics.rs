use forky_play::*;
use sweet::*;

sweet! {
	
	it "works" {
		app::init()
			.add_plugin(base::BasicPlugin)
			.forky_exit_after(2)
			.run();
	}
}


