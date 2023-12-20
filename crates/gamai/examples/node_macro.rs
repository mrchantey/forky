use gamai::prelude::*;



pub fn main() {}

#[action(system=foo)]
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct Foo {
	score: Score,
}

fn foo() {}
