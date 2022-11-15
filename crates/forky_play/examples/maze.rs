use forky_play::{maze::MazePlugin, *};

fn main() {
	app::init()
		.add_plugin(MazePlugin)
		// .forky_exit_after(2)
		.run();
}
