use forky_play::*;
use sweet::*;

sweet! {

	it "works" {
		app::init()
			// .add_plugin(base::BasicPlugin)
			// .forky_surrender_focus()
			.forky_exit_after(3)
			.run();
	}
}
