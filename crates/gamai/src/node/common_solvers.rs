// pub mod common_solvers {
use crate::*;
use bevy::prelude::*;
use std::marker::PhantomData;

#[node_system]
pub fn first_valid_edge<N: AiNode>(
	mut commands: Commands,
	mut query: Query<N::ChildrenQuery>,
) {
	let entities = N::edges(&mut query);
	for (entity, edges) in entities.iter() {
		for (index, edge) in edges.iter().enumerate() {
			if *edge != EdgeState::Fail {
				N::set_action(&mut commands, *entity, index);
				continue; //skip other edges, go to next entity
			}
		}
	}
}

#[choice_system]
pub fn edge_always_pass<C: Choice>(mut query: Query<&mut ChoiceEdgeState<C>>) {
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Pass;
	}
}
#[choice_system]
pub fn edge_always_fail<C: Choice>(mut query: Query<&mut ChoiceEdgeState<C>>) {
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Fail;
	}
}
#[choice_system]
pub fn action_print<C: Choice>(mut query: Query<&mut ChoiceActionState<C>>) {
	for action in query.iter_mut() {
		println!("Action: Running {:?}", action);
	}
}
#[choice_system]
pub fn action_noop<C: Choice>(_phantom: PhantomData<C>) {}
