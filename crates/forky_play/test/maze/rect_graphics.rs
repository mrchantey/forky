use forky_play::*;
use sweet::*;
// use crate::maze::plugin::MazePlugin;
sweet! {

	it "works" {
		app::init()
			.add_plugin(maze::MazePlugin)
			// .forky_exit_after(2)
			.run();
	}
}
