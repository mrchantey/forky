use gamai::ui::FieldUiRoot;
use gamai_macros::FieldUi;
// use gamai::prelude::*;
use strum_macros::Display;
use strum_macros::EnumIter;
// use gamai_macros::*;

fn main() {
	let foo = Foo {
		health: 100,
		// attack: AttackType::Punch {
		// 	left: 32,
		// 	right: 88,
		// },
		attack: AttackType::Ranged(32., 88),
	};
	let root = FieldUiRoot::new(foo);
	println!("{}", root.get_ui().into_string_tree());

	// match foo.attack {
	// 	AttackType::Melee => println!("melee"),
	// 	#[allow(unused_variables)]
	// 	AttackType::Ranged(range, speed) => println!("ranged: {} {}", range, speed),
	// }
}

#[derive(Clone, EnumIter, Display, FieldUi)]
pub enum AttackType {
	Melee,
	Ranged(
		#[slider(min = 0., max = 1., step = 0.01)] f32,
		#[number(step = 2)] u32,
	),
	Punch {
		left: u32,
		right: u32,
	},
}

#[derive(Clone, FieldUi)]
pub struct Foo {
	#[slider(min = 0, max = 100, step = 1)]
	health: u32,
	attack: AttackType,
}
