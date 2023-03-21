use forky_play::{app, maze::MazePlugin, utility, OptI32X};
use sweet::*;
// use crate::maze::plugin::MazePlugin;
sweet! {

	it skip "works" {
		app::init()
			.add_plugin(MazePlugin)
			.add_startup_system(utility::surrender_focus)
			.forky_exit_after(2.)
			.run();

	}
}
