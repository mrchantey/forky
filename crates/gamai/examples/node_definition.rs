#![feature(
	associated_type_bounds,
	return_position_impl_trait_in_trait,
	// generic_const_exprs
)]
//this example is used for macro expansion, for usage see the `tests` directory
// use bevy_ecs::schedule::Schedule;
// use gamai::*;

// type Parent = Node1<empty_node, empty_node, 0, 0, 0, 0, 0, Child>;
// type Child = Node0<empty_node, empty_node, 1, 0, 1, 0, 0>;
fn main() {
	// let (bar,baz) = foo;
	// let mut schedule = Schedule::new();
	// Parent::add_systems(&mut schedule);
	// let mut world = World::new();
	// world.spawn(Parent::default());
}
gamai::define_node!(0);
gamai::define_node!(1);
// gamai::define_node!(2);
// gamai::define_node!(3);
