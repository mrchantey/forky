use forky_play::*;

fn main() {
	app::init()
		.add_plugin(maze::MazePlugin)
		// .forky_exit_after(2)
		.run();
}
