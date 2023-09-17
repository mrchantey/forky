// pub mod common_solvers {
use crate::*;
use bevy::prelude::*;
#[agent_system]
pub fn solver_first_valid<A: Agent>(
	mut commands: Commands,
	mut query: Query<A::Items>,
) {
	let choices = A::factors(&mut query);
	for (entity, factors) in choices.iter() {
		for (index, factor) in factors.iter().enumerate() {
			if *factor != FactorState::Fail {
				println!("Solver: Setting Action: {index} - {:?}", factor);
				A::set_action(&mut commands, *entity, index);
				continue; //go to next entity
			}
		}
	}
}

#[choice_system]
pub fn factor_always_pass<C: Choice>(
	mut query: Query<&mut ChoiceFactorState<C>>,
) {
	for mut factor in query.iter_mut() {
		**factor = FactorState::Pass;
		println!("Factor: Set Pass {:?}", factor);
	}
}
#[choice_system]
pub fn factor_always_fail<C: Choice>(
	mut query: Query<&mut ChoiceFactorState<C>>,
) {
	for mut factor in query.iter_mut() {
		**factor = FactorState::Fail;
		println!("Factor: Set Fail {:?}", factor);
	}
}
#[choice_system]
pub fn action_print<C:Choice>(
	mut query: Query<&mut ChoiceActionState<C>>,
) {
	for action in query.iter_mut() {
		println!("Action: Running {:?}", action);
	}
}
