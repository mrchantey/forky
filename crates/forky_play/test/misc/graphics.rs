use forky_play::*;
use sweet::*;

sweet! {

	it skip "works" {
		app::init()
			// .forky_surrender_focus()
			.forky_exit_after(1)
			.run();
	}
}
