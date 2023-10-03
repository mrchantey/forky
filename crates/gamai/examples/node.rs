use gamai::*;

//this example is used for macro expansion, for usage see the `tests` directory
fn some_system() {}

fn my_node() -> impl AiNode {
	let node0 =
		Node0::<0, 0, 0, 0, 0, _, _, _, _>::new(|| some_system, || some_system);
	Node1::<0, 0, 0, 0, 0, _, _, _, _, _>::new(
		|| some_system,
		|| some_system,
		node0,
	)
}


fn main() {
	let _bundle = AiBundle::new(my_node);
	let _plugin = AiPlugin::new(my_node);
}
