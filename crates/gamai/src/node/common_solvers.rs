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
				N::set_child_node_state(&mut commands, *entity, index);
				continue; //skip other edges, go to next entity
			}
		}
	}
}

#[edge_system]
pub fn edge_always_pass<C: Choice>(mut query: Query<&mut ChildEdgeState<C>>) {
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Pass;
	}
}
#[edge_system]
pub fn edge_always_fail<C: Choice>(mut query: Query<&mut ChildEdgeState<C>>) {
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Fail;
	}
}
#[edge_system]
pub fn print_on_run<C: Choice>(mut query: Query<&mut ChildNodeState<C>>) {
	for node in query.iter_mut() {
		println!("NodeSystem: Running {:?}", node);
	}
}
#[edge_system]
pub fn noop_node<C: Choice>(_phantom: PhantomData<C>) {}
