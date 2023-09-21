use bevy_app::App;
use gamai::*;
use sweet::*;
type MyTree = gamai::tree!(<empty_node/>);

sweet! {
	it "works" {
		let mut app = App::new();
		app.add_plugins(MyTree::plugin());
		app.world.spawn(MyTree::bundle());
		app.update();
	}
}
