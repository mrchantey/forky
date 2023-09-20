#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
use bevy_ecs::prelude::*;
//this example is used for macro expansion, for usage see the `tests` directory
use gamai::*;

type MyGraph = gamai::graph!(
	<first_valid_edge edge=edge_always_pass>
		<noop_node edge=edge_always_fail/>
		<noop_node edge=edge_always_pass/>
	</first_valid_edge>
);

fn main() {
	let mut world = World::new();
	world.spawn(MyGraph::default());
	let mut schedule = Schedule::new();
	MyGraph::build(&mut schedule);
	// println!("{:?}", out);
}
