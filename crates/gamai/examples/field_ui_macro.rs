use gamai::node::Score;
use gamai_macros::field_ui;

fn main() {}



#[field_ui]
pub struct Foo {
	health: u32,
	score: Score,
}
