#![feature(associated_const_equality, return_position_impl_trait_in_trait)]
use gamai::*;



fn main() {}



#[tree_builder]
fn foobar() -> impl AiNode {
	Node0::<CHILD_INDEX, Parent, _, _, _, _>::new(|| || {}, || || {})
}
