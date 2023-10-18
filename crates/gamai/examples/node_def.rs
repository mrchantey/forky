#![feature(associated_const_equality, generic_const_exprs)]
#![allow(incomplete_features)]
//this example is used for macro expansion, for usage see the `tests` directory
fn main() {}
// gamai::define_node!(0);
gamai::define_node!(1);
// gamai::define_node!(2);


// // trait MyTrait<N, Q> {
// // 	fn foobar()
// // }

// impl<'a, N, Q> MyTrait<N, Q>
// 	for <<N as AiNode>::ChildQueryOpt<ActionResult> as WorldQuery>::Item<'a>
// where
// 	N: AiNode,
// 	Q: WorldQuery<Item<'a> = ActionResult>,
// {
// 	// implementation goes here
// }
