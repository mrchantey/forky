// pub mod common_solvers {
use crate::*;
use bevy_ecs::prelude::*;
use std::marker::PhantomData;

#[node]
pub fn noop_node<N: AiNode>() {}

#[node]
pub fn first_valid_edge<N: AiNode>(
	mut commands: Commands,
	mut query: Query<N::ChildrenQuery>,
) {
	let entities = N::edges(&mut query);
	for (entity, edges) in entities.iter() {
		for (index, edge) in edges.iter().enumerate() {
			if *edge != EdgeState::Fail {
				N::set_child_node_state(&mut commands, *entity, index).unwrap();
				continue; //skip other edges, go to next entity
			}
		}
	}
}

#[node]
pub fn edge_always_pass<N: AiNode>(mut query: Query<&mut ChildEdgeState<N>>) {
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Pass;
	}
}
#[node]
pub fn edge_always_fail<N: AiNode>(mut query: Query<&mut ChildEdgeState<N>>) {
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Fail;
	}
}
#[node]
pub fn print_on_run<N: AiNode>(mut query: Query<&mut ChildNodeState<N>>) {
	for node in query.iter_mut() {
		println!("NodeSystem: Running {:?}", node);
	}
}
#[node]
pub fn noop_edge<N: AiNode>(_phantom: PhantomData<N>) {}
