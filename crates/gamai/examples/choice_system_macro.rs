#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
use bevy::prelude::*;
use gamai::*;

fn main() {}
// #[choice_system]
// fn my_system<T: DerefFactorState>(query: Query<&T>) {
// 	for state in query.iter() {
// 		println!("state: {:?}", state);
// 	}
// }

#[choice_system]
pub fn action_print<C: Choice>(mut query: Query<&mut ChoiceActionState<C>>) {
	for action in query.iter_mut() {
		println!("Action: Running {:?}", action);
	}
}
