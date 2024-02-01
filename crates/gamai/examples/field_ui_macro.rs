use gamai::prelude::*;
use strum_macros::Display;
use strum_macros::EnumIter;
// use gamai_macros::*;

fn main() {
	let foo = Foo {
		health: 100,
		attack: AttackType::Punch {
			left: 32,
			right: 88,
		},
		// attack: AttackType::Ranged(32., 88),
	};
	let ui = foo.into_field_ui_root();
	println!("{}", ui.into_string_tree());

	// match foo.attack {
	// 	AttackType::Melee => println!("melee"),
	// 	#[allow(unused_variables)]
	// 	AttackType::Ranged(range, speed) => println!("ranged: {} {}", range, speed),
	// }
}



#[derive(Clone, FieldUi)]
pub struct Foo {
	health: u32,
	attack: AttackType,
}
#[derive(Clone, EnumIter, Display, FieldUi)]
pub enum AttackType {
	Melee,
	Ranged(f32, u32),
	Punch { left: u32, right: u32 },
}
