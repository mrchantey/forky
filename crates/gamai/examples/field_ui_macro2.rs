use gamai::prelude::*;
use gamai_macros::FieldUi;
// use gamai::prelude::*;
use strum_macros::Display;
use strum_macros::EnumIter;
// use gamai_macros::*;

fn main() {}

#[derive(Clone, EnumIter, Display, FieldUi)]
// #[hide_ui]
pub enum AttackType {
	Foo,
	Bar { value: u32 },
	Baz,
}
