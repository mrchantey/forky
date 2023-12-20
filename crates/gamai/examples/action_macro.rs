use gamai::prelude::*;



pub fn main() {}

#[action(system=foo)]
#[derive(Clone, Serialize, Deserialize, Component)]
pub struct Foo {
	health: u32,
	#[shared]
	score: Score,
}

fn foo() {}
