use gamai::prelude::*;



pub fn main() {}

#[action(system=foo)]
#[derive(Default)]
pub struct Foo {
	health: u32,
	#[shared]
	score: Score,
}

fn foo() {}
