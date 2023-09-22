// pub mod common_solvers {
use crate::*;
use bevy_ecs::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Default, Clone)]
#[allow(non_camel_case_types)]
pub struct empty_node;

impl IntoNodeSystem for empty_node {
	fn add_node_system<N: AiNode>(
		_schedule: &mut Schedule,
		_set: impl SystemSet,
		_config: &NodeSystemConfig,
	) {
	}
}

#[node_system]
pub fn first_valid_edge<N: AiNode>(
	mut commands: Commands,
	mut query: Query<N::ChildQuery, With<ChildNodeState<N>>>,
) {
	for node in query.iter_mut() {
		let mut children = N::children(node);
		for child in children.iter_mut() {
			if **child.edge != EdgeState::Fail {
				// println!("first_valid_edge: setting node state..");
				child.set_node_state(&mut commands, Some(NodeState::Running));
			}
		}
	}
}

#[node_system]
pub fn edge_always_pass<N: AiNode>(mut query: Query<&mut ChildEdgeState<N>>) {
	// println!("edge_always_pass: Running");
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Pass;
	}
}
#[node_system]
pub fn edge_always_fail<N: AiNode>(mut query: Query<&mut ChildEdgeState<N>>) {
	for mut edge in query.iter_mut() {
		**edge = EdgeState::Fail;
	}
}
#[node_system]
pub fn print_on_run<N: AiNode>(mut query: Query<&mut ChildNodeState<N>>) {
	// println!("print_on_run: running..");
	for node in query.iter_mut() {
		println!("NodeSystem: Running {:?}", node);
	}
}
#[node_system]
pub fn noop_edge<N: AiNode>(_phantom: PhantomData<N>) {}
