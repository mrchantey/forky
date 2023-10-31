#![feature(associated_const_equality, generic_const_exprs)]
#![allow(incomplete_features)]
use gamai::*;
//this example is for macro expansion, for usage see the `tests` directory

#[action]
fn my_system<N: AiNode>() {}

fn main() {
	let _bundle = TreeBundle::new(MyTree);

}

#[tree_builder]
pub fn OtherTree(
	foo: u32,
	#[default = 3] bar: u8,
	#[default] bazz: Option<&'static str>,
) -> impl TreeElement {
	tree! {
		<my_system props=(foo,bar,bazz)/>
	}
}
#[tree_builder]
pub fn EmptyTree() -> impl TreeElement {
	tree! {
		<my_system/>
	}
}


#[tree_builder]
pub fn MyTree() -> impl TreeElement {
	tree! {
		<OtherTree foo=3/>
		// <EmptyTree foo=3/>
		// <my_system props=(foo,bar,bazz)/>
	}
}
