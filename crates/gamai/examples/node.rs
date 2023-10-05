#![feature(return_position_impl_trait_in_trait, associated_const_equality)]
//this example is used for compilation checks, for usage see the `tests` directory
use gamai::*;

fn my_bevy_system() {}

#[node_system]
fn my_node_system<N: AiNode>() {}

fn my_node() -> impl AiNode {
	let node0 = Node0::<0, RootParent<0>, _, _, _, _>::new(
		my_node_system,
		my_node_system,
	);
	// node0
	Node1::<0, RootParent<0>, _, _, _, _, _>::new(
		|| my_bevy_system,
		|| my_bevy_system,
		node0,
	)
}

fn main() {
	let _bundle = AiBundle::new(my_node);
	let _plugin = AiPlugin::new(my_node);
}
