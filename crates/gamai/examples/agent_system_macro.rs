#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
use bevy::prelude::*;
use gamai::*;

fn main() {}

#[agent_system]
fn first_valid_solver<A: Agent>(
	mut commands: Commands,
	mut query: Query<A::Items>,
) {
	let choices = A::factors(&mut query);
	for (entity, factors) in choices.iter() {
		for (index, factor) in factors.iter().enumerate() {
			if *factor != FactorState::Fail {
				A::set_action(&mut commands, *entity, index);
				return;
			}
		}
		// println!("{:?}: {:?}", entity, factors);
	}
}
