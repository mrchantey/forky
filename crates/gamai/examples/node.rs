#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
//this example is used for macro expansion, for usage see the `tests` directory

fn main() {}

#[gamai::node(num_choices=20)]
// #[gamai::node(num_edges(20))]
fn my_node() {}
